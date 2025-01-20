use crate::protocol::{Message, MessageBody};
use std::default::Default;

#[derive(Default)]
pub struct Node {
    pub id: String,
    // TODO not actually sure what "ids" is for.
    pub ids: Vec<String>,
    pub neighbor_ids: Option<Vec<String>>,
    pub next_msg_id: usize,
}

impl Node {
    pub fn log(&mut self, text: &str) -> () {
        eprintln!("{}", text);
    }

    fn send(&mut self, msg: &Message) -> () {
        self.next_msg_id += 1;

        self.log(&format!("Sending reply: {msg:?}"));

        println!("{}", serde_json::to_string(&msg).unwrap());

        self.log(&format!("Finished sending reply: {msg:?}"));
    }
}

// TODO clean up these send_*_reply functions and just use a handle function with a big match on the message body type.
pub fn send_init_reply(node: &mut Node, original_msg: Message) {
    match original_msg.body {
        MessageBody::Init { msg_id, .. } => {
            let reply = Message {
                src: node.id.clone(),
                dest: original_msg.src,
                body: MessageBody::InitOk {
                    msg_id: Some(node.next_msg_id),
                    in_reply_to: msg_id,
                },
            };
            node.send(&reply);
        }
        _ => panic!("Cannot send an init_ok reply to a non-init message: {original_msg:?}"),
    }
}

pub fn send_echo_reply(node: &mut Node, original_msg: Message) {
    match original_msg.body {
        MessageBody::Echo { msg_id, echo, .. } => {
            let reply = Message {
                src: node.id.clone(),
                dest: original_msg.src,
                body: MessageBody::EchoOk {
                    msg_id: Some(node.next_msg_id),
                    in_reply_to: msg_id,
                    echo,
                },
            };

            node.send(&reply);
        }
        _ => panic!("Cannot send an echo reply to a non-echo message: {original_msg:?}"),
    }
}

pub fn send_topology_reply(node: &mut Node, original_msg: Message) {
    match original_msg.body {
        MessageBody::Topology { msg_id, .. } => {
            let reply = Message {
                src: node.id.clone(),
                dest: original_msg.src,
                body: MessageBody::TopologyOk {
                    msg_id: Some(node.next_msg_id),
                    in_reply_to: msg_id,
                },
            };

            node.send(&reply);
        }
        _ => panic!("Cannot send a topology reply to a non-topology message: {original_msg:?}"),
    }
}
