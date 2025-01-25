mod node;
mod protocol;

use node::{log, send, send_ok_reply, Node};
use protocol::{Message, MessageBody};
use std::default::Default;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let stdin = io::stdin();

    let node = Arc::new(Mutex::<Node>::new(Default::default()));

    let retry_these = Arc::new(Mutex::new(Vec::<Message>::new()));

    // Spawn a thread that keeps checking retry_these and sends those messages.
    std::thread::spawn({
        let retry_these = retry_these.clone();
        move || loop {
            for message in retry_these.lock().unwrap().iter() {
                send(message);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    for line in stdin.lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();

            std::thread::spawn({
                // TODO fix the double shadowing nonsense
                let node = node.clone();
                let retry_these = retry_these.clone();
                move || {
                    match &msg.body {
                        MessageBody::Init {
                            node_id, node_ids, ..
                        } => {
                            let mut node = node.lock().unwrap();
                            log(&format!("Got Init message: {msg:?}"));

                            // Initialize our node.
                            node.id = node_id.clone();
                            node.ids = node_ids.clone();

                            // Reply back with an init_ok.
                            send_ok_reply(&mut node, msg);
                        }
                        MessageBody::Echo { .. } => {
                            let mut node = node.lock().unwrap();
                            log(&format!("Got Echo message: {msg:?}"));

                            send_ok_reply(&mut node, msg);
                        }
                        MessageBody::Topology { topology, .. } => {
                            let mut node = node.lock().unwrap();
                            log(&format!("Got topology message: {msg:?}"));

                            if let Some(neighbors) = topology.get(&node.id) {
                                node.neighbor_ids = Some(neighbors.clone());
                                log(&format!("Setting neighbors to {:?}", neighbors));
                            }

                            send_ok_reply(&mut node, msg);
                        }
                        MessageBody::Broadcast {
                            msg_id,
                            message: number,
                            ..
                        } => {
                            let mut node = node.lock().unwrap();
                            log(&format!("Got broadcast message: {msg:?}"));

                            // If we haven't seen this broadcast message before
                            if !node.messages.contains(number) {
                                // Record this message.
                                node.messages.insert(*number);

                                // Gossip to neighbors except the src of this originating broadcast.
                                if let Some(neighbors) = node.neighbor_ids.clone() {
                                    for neighbor in
                                        neighbors.iter().filter(|neighbor| **neighbor != msg.src)
                                    {
                                        let message: Message = Message {
                                            src: node.id.clone(),
                                            dest: neighbor.clone(),
                                            body: MessageBody::Broadcast {
                                                msg_id: Some(node.next_msg_id),
                                                in_reply_to: *msg_id,
                                                message: *number,
                                            },
                                        };

                                        // Send the gossip message to the neighbor. This also increments node.next_msg_id.
                                        send(&message);
                                        node.next_msg_id += 1;

                                        // TODO this is a nested lock and is error prone. Avoid this pattern.
                                        // Add this message to be retried later.
                                        retry_these.lock().unwrap().push(message);
                                    }
                                }
                                // No neighbors, do nothing.
                            }

                            // Reply with an ok.
                            send_ok_reply(&mut node, msg);
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
                            let mut node = node.lock().unwrap();
                            log(&format!("Got read message: {msg:?}"));

                            send_ok_reply(&mut node, msg);
                        }
                        _ => todo!(),
                    };
                }
            });
        }
    }
}
