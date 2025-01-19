mod node;
mod protocol;

use std::io;
use std::io::BufRead;

use crate::node::{send_echo_reply, send_init_reply, Node};
use crate::protocol::{Message, MessageBody};
use std::default::Default;

fn main() {
    let stdin = io::stdin();

    let mut node: Node = Default::default();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();
            match &msg.body {
                MessageBody::Init { node_id, .. } => {
                    eprintln!("Got Init message: {msg:?}");

                    // Initialize our node.
                    node.id = node_id.clone();

                    // Reply back with an init_ok.
                    send_init_reply(&mut node, msg);
                }
                MessageBody::Echo { .. } => {
                    eprintln!("Got Echo message: {msg:?}");

                    send_echo_reply(&mut node, msg);
                }
                _ => todo!(),
            };
        }
    }
}
