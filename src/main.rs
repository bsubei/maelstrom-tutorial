mod node;
mod protocol;

use node::{send_echo_reply, send_init_reply, send_topology_reply, Node};
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
                        send_init_reply(&mut node, msg);
                    }
                    MessageBody::Echo { .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got Echo message: {msg:?}"));

                        send_echo_reply(&mut node, msg);
                    }
                    MessageBody::Topology { topology, .. } => {
                        let mut node = node_ref.lock().unwrap();
                        node.log(&format!("Got topology message: {msg:?}"));

                        if let Some(neighbors) = topology.get(&node.id) {
                            node.neighbor_ids = Some(neighbors.clone());
                            node.log(&format!("Setting neighbors to {:?}", neighbors));
                        }

                        send_topology_reply(&mut node, msg);
                    }
                    _ => todo!(),
                };
            });
        }
    }
}
