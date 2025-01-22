use crate::protocol::{Message, MessageBody};
use std::collections::{HashMap, HashSet};
use std::default::Default;

#[derive(Default)]
pub struct Node {
    pub id: String,
    // TODO not actually sure what "ids" is for.
    pub ids: Vec<String>,
    pub neighbor_ids: Option<Vec<String>>,
    pub next_msg_id: usize,
    // The list of messages we've seen.
    pub messages: HashSet<usize>,
}

impl Node {
    pub fn log(&mut self, text: &str) -> () {
        eprintln!("{}", text);
    }

    pub fn send(&mut self, msg: &Message) -> () {
        self.log(&format!("Sending reply: {msg:?}"));

        println!("{}", serde_json::to_string(&msg).unwrap());

        self.log(&format!("Finished sending reply: {msg:?}"));
    }
}

pub fn send_ok_reply(node: &mut Node, original_msg: Message) {
    let reply = match original_msg.body {
        MessageBody::Init { msg_id, .. } => Message {
            src: node.id.clone(),
            dest: original_msg.src,
            body: MessageBody::InitOk {
                msg_id: Some(node.next_msg_id),
                in_reply_to: msg_id,
            },
        },
        MessageBody::Echo { msg_id, echo, .. } => Message {
            src: node.id.clone(),
            dest: original_msg.src,
            body: MessageBody::EchoOk {
                msg_id: Some(node.next_msg_id),
                in_reply_to: msg_id,
                echo,
            },
        },
        MessageBody::Topology { msg_id, .. } => Message {
            src: node.id.clone(),
            dest: original_msg.src,
            body: MessageBody::TopologyOk {
                msg_id: Some(node.next_msg_id),
                in_reply_to: msg_id,
            },
        },
        MessageBody::Broadcast { msg_id, .. } => Message {
            src: node.id.clone(),
            dest: original_msg.src,
            body: MessageBody::BroadcastOk {
                msg_id: Some(node.next_msg_id),
                in_reply_to: msg_id,
            },
        },
        MessageBody::Read { msg_id, .. } => Message {
            src: node.id.clone(),
            dest: original_msg.src,
            body: MessageBody::ReadOk {
                msg_id: Some(node.next_msg_id),
                in_reply_to: msg_id,
                messages: node.messages.iter().cloned().collect(),
            },
        },
        _ => unimplemented!("Cannot send a reply to this message: {original_msg:?}"),
    };
    node.send(&reply);
    node.next_msg_id += 1;
}
