mod node;
mod protocol;

use node::{log, send, send_ok_reply, Node};
use protocol::{Message, MessageBody};
use std::default::Default;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let stdin = io::stdin();

    let node = Arc::new(Mutex::<Node>::new(Default::default()));
    let retry_these = Arc::new(Mutex::new(Vec::<Message>::new()));

    for line in stdin.lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();

            // Spawn a new task to handle the incoming message.
            tokio::spawn({
                let node = node.clone();
                let retry_these = retry_these.clone();
                async move {
                    match &msg.body {
                        MessageBody::Init {
                            node_id, node_ids, ..
                        } => {
                            log(&format!("Got Init message: {msg:?}"));
                            {
                                let mut node_g = node.lock().unwrap();

                                // Initialize our node.
                                node_g.id = node_id.clone();
                                node_g.ids = node_ids.clone();
                            }

                            // Reply back with an init_ok.
                            send_ok_reply(node, msg).await;
                        }
                        MessageBody::Echo { .. } => {
                            log(&format!("Got Echo message: {msg:?}"));
                            send_ok_reply(node, msg).await;
                        }
                        MessageBody::Topology { topology, .. } => {
                            {
                                let mut node_g = node.lock().unwrap();
                                log(&format!("Got topology message: {msg:?}"));

                                if let Some(neighbors) = topology.get(&node_g.id) {
                                    node_g.neighbor_ids = Some(neighbors.clone());
                                    log(&format!("Setting neighbors to {:?}", neighbors));
                                }
                            }

                            send_ok_reply(node, msg).await;
                        }
                        MessageBody::Broadcast {
                            msg_id,
                            message: number,
                            ..
                        } => {
                            // Lock the node mutex while we figure out which neighbors we should send to, then unlock.
                            let neighbors_to_send_to = {
                                let node_g = node.lock().unwrap();
                                log(&format!("Got broadcast message: {msg:?}"));

                                // TODO refactor to be less verbose
                                if let Some(neighbors) = &node_g.neighbor_ids {
                                    Some(
                                        neighbors
                                            .iter()
                                            .filter(|n| **n != msg.src)
                                            .cloned()
                                            .collect::<Vec<_>>(),
                                    )
                                } else {
                                    None
                                }
                            };

                            // Go over each neighbor we should send to. In each iteration:
                            // 1. lock the node mutex while we construct the Message to send, then unlock.
                            // 2. Then, send the message (async).
                            // 3. When we come back, lock the node mutex to increment the msg id. Also lock retry_these to push this message (warning: nested mutex, error prone).
                            if let Some(neighbors) = neighbors_to_send_to {
                                for neighbor in neighbors {
                                    let message = {
                                        let mut node_g = node.lock().unwrap();
                                        // If we haven't seen this broadcast message before
                                        if !node_g.messages.contains(number) {
                                            // Record this message.
                                            node_g.messages.insert(*number);
                                            // Gossip it to neighbors.
                                            node_g.next_msg_id += 1;
                                            Some(Message {
                                                src: node_g.id.clone(),
                                                dest: neighbor.clone(),
                                                body: MessageBody::Broadcast {
                                                    msg_id: Some(node_g.next_msg_id),
                                                    in_reply_to: *msg_id,
                                                    message: *number,
                                                },
                                            })
                                        } else {
                                            None
                                        }
                                    };
                                    if let Some(message) = message {
                                        send(&message).await;
                                        // Add this message to be retried later.
                                        retry_these.lock().unwrap().push(message);
                                    }
                                }
                                // Reply with an ok.
                                send_ok_reply(node, msg).await;
                            }

                            // Finally, keep trying to retry these messages until we get receive replies.
                            loop {
                                tokio::time::sleep(Duration::from_millis(500)).await;
                                let messages_to_retry = { retry_these.lock().unwrap().clone() };
                                for message in messages_to_retry {
                                    send(&message).await;
                                }
                            }
                        }
                        MessageBody::BroadcastOk { in_reply_to, .. } => {
                            log(&format!("Got broadcast_ok message: {msg:?}"));
                            // Remove this message from the retry_these list.
                            if let Some(in_reply_to) = in_reply_to {
                                retry_these
                                    .lock()
                                    .unwrap()
                                    .retain(|msg| msg.get_msg_id().unwrap() != *in_reply_to);
                            }
                        }
                        MessageBody::Read { .. } => {
                            log(&format!("Got read message: {msg:?}"));
                            send_ok_reply(node, msg).await;
                        }
                        _ => todo!(),
                    };
                }
            });
        }
    }
}
