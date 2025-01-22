mod node;
mod protocol;

use node::{send_ok_reply, Node};
use protocol::{Message, MessageBody};
use std::default::Default;
use std::io;
use std::sync::{Arc, Mutex};

fn main() {
    let stdin = io::stdin();

    let node = Arc::new(Mutex::<Node>::new(Default::default()));

    for line in stdin.lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();

            let node_ref = node.clone();
            std::thread::spawn(move || {
                match &msg.body {
                    MessageBody::Init {
                        node_id, node_ids, ..
                    } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got Init message: {msg:?}"));

                        // Initialize our node.
                        node.id = node_id.clone();
                        node.ids = node_ids.clone();

                        // Reply back with an init_ok.
                        send_ok_reply(&mut node, msg);
                    }
                    MessageBody::Echo { .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got Echo message: {msg:?}"));

                        send_ok_reply(&mut node, msg);
                    }
                    MessageBody::Topology { topology, .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got topology message: {msg:?}"));

                        if let Some(neighbors) = topology.get(&node.id) {
                            node.neighbor_ids = Some(neighbors.clone());
                            node.log(&format!("Setting neighbors to {:?}", neighbors));
                        }

                        send_ok_reply(&mut node, msg);
                    }
                    MessageBody::Broadcast {
                        msg_id, message, ..
                    } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got broadcast message: {msg:?}"));

                        if !node.messages.contains(message) {
                            // Record this message.
                            node.messages.insert(message.clone());

                            // Gossip to neighbors except the src of this originating broadcast.
                            let messages = if let Some(neighbors) = &node.neighbor_ids {
                                neighbors
                                    .iter()
                                    .filter(|neighbor| **neighbor != msg.src)
                                    .map(|neighbor| Message {
                                        src: node.id.clone(),
                                        dest: neighbor.clone(),
                                        body: MessageBody::Broadcast {
                                            msg_id: Some(node.next_msg_id),
                                            in_reply_to: *msg_id,
                                            message: message.clone(),
                                        },
                                    })
                                    .collect()
                            } else {
                                vec![]
                            };
                            for message in messages {
                                node.send(&message);
                            }
                        }

                        // Reply with an ok.
                        send_ok_reply(&mut node, msg);
                    }
                    MessageBody::BroadcastOk { .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got broadcast_ok message: {msg:?}"));
                    }
                    MessageBody::Read { .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got read message: {msg:?}"));

                        send_ok_reply(&mut node, msg);
                    }
                    _ => todo!(),
                };
            });
        }
    }
}
