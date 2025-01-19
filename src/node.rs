use crate::protocol::{Message, MessageBody};
use std::default::Default;

#[derive(Default)]
pub struct Node {
    pub id: String,
    pub next_msg_id: usize,
}

impl Node {}

fn send(node: &mut Node, msg: &Message) -> () {
    node.next_msg_id += 1;

    println!("{}", serde_json::to_string(&msg).unwrap());
}

pub fn send_init_reply(mut node: &mut Node, original_msg: Message) {
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
            eprintln!("Sending reply: {reply:?}");
            send(&mut node, &reply);
            eprintln!("Finished sending reply: {reply:?}");
        }
        _ => panic!("Cannot send an init_ok reply to a non-init message: {original_msg:?}"),
    }
}

pub fn send_echo_reply(mut node: &mut Node, original_msg: Message) {
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

            eprintln!("Sending reply: {reply:?}");
            send(&mut node, &reply);
            eprintln!("Finished sending reply: {reply:?}");
        }
        _ => panic!("Cannot send an echo reply to a non-echo message: {original_msg:?}"),
    }
}
