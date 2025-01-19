mod node;
mod protocol;

use crate::node::{send_echo_reply, send_init_reply, Node};
use crate::protocol::{Message, MessageBody};
use std::default::Default;
use std::io;

fn main() {
    let stdin = io::stdin();

    let mut node: Node = Default::default();

    for line in stdin.lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();
            match &msg.body {
                MessageBody::Init { node_id, .. } => {
                    node.log(&format!("Got Init message: {msg:?}"));

                    // Initialize our node.
                    node.id = node_id.clone();

                    // Reply back with an init_ok.
                    send_init_reply(&mut node, msg);
                }
                MessageBody::Echo { .. } => {
                    node.log(&format!("Got Echo message: {msg:?}"));

                    send_echo_reply(&mut node, msg);
                }
                _ => todo!(),
            };
        }
    }
}
