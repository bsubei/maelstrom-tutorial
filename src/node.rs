use crate::protocol::{Message, MessageBody};
use std::collections::HashSet;
use std::default::Default;
use std::sync::{Arc, Mutex};

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

pub fn log(text: &str) -> () {
    eprintln!("{}", text);
}

pub async fn send(msg: &Message) -> () {
    // TODO update all these stdout/stderr to use async

    log(&format!("Sending reply: {msg:?}"));

    println!("{}", serde_json::to_string(&msg).unwrap());

    log(&format!("Finished sending reply: {msg:?}"));
}

pub async fn send_ok_reply(node: Arc<Mutex<Node>>, original_msg: Message) {
    let reply = {
        let mut node_g = node.lock().unwrap();
        node_g.next_msg_id += 1;
        match original_msg.body {
            MessageBody::Init { msg_id, .. } => Message {
                src: node_g.id.clone(),
                dest: original_msg.src,
                body: MessageBody::InitOk {
                    msg_id: Some(node_g.next_msg_id),
                    in_reply_to: msg_id,
                },
            },
            MessageBody::Echo { msg_id, echo, .. } => Message {
                src: node_g.id.clone(),
                dest: original_msg.src,
                body: MessageBody::EchoOk {
                    msg_id: Some(node_g.next_msg_id),
                    in_reply_to: msg_id,
                    echo,
                },
            },
            MessageBody::Topology { msg_id, .. } => Message {
                src: node_g.id.clone(),
                dest: original_msg.src,
                body: MessageBody::TopologyOk {
                    msg_id: Some(node_g.next_msg_id),
                    in_reply_to: msg_id,
                },
            },
            MessageBody::Broadcast { msg_id, .. } => Message {
                src: node_g.id.clone(),
                dest: original_msg.src,
                body: MessageBody::BroadcastOk {
                    msg_id: Some(node_g.next_msg_id),
                    in_reply_to: msg_id,
                },
            },
            MessageBody::Read { msg_id, .. } => Message {
                src: node_g.id.clone(),
                dest: original_msg.src,
                body: MessageBody::ReadOk {
                    msg_id: Some(node_g.next_msg_id),
                    in_reply_to: msg_id,
                    messages: node_g.messages.iter().cloned().collect(),
                },
            },
            _ => unimplemented!("Cannot send a reply to this message: {original_msg:?}"),
        }
    };
    send(&reply).await;
}
