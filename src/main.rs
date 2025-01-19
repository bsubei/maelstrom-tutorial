use std::default::Default;
use std::io;
use std::io::BufRead;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MessageBody {
    Init {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
    },
    Echo {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        echo: String,
    },
    EchoOk {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        echo: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[derive(Default)]
struct NodeState {
    id: String,
    next_msg_id: usize,
}

fn send(node_state: &mut NodeState, msg: &Message) -> () {
    node_state.next_msg_id += 1;

    println!("{}", serde_json::to_string(&msg).unwrap());
}

fn send_init_reply(mut node_state: &mut NodeState, original_msg: Message) {
    match original_msg.body {
        MessageBody::Init { msg_id, .. } => {
            let reply = Message {
                src: node_state.id.clone(),
                dest: original_msg.src,
                body: MessageBody::InitOk {
                    msg_id: Some(node_state.next_msg_id),
                    in_reply_to: msg_id,
                },
            };
            eprintln!("Sending reply: {reply:?}");
            send(&mut node_state, &reply);
            eprintln!("Finished sending reply: {reply:?}");
        }
        _ => panic!("Cannot send an init_ok reply to a non-init message: {original_msg:?}"),
    }
}

fn send_echo_reply(mut node_state: &mut NodeState, original_msg: Message) {
    match original_msg.body {
        MessageBody::Echo { msg_id, echo, .. } => {
            let reply = Message {
                src: node_state.id.clone(),
                dest: original_msg.src,
                body: MessageBody::EchoOk {
                    msg_id: Some(node_state.next_msg_id),
                    in_reply_to: msg_id,
                    echo,
                },
            };

            eprintln!("Sending reply: {reply:?}");
            send(&mut node_state, &reply);
            eprintln!("Finished sending reply: {reply:?}");
        }
        _ => panic!("Cannot send an echo reply to a non-echo message: {original_msg:?}"),
    }
}

fn main() {
    let stdin = io::stdin();

    let mut node_state: NodeState = Default::default();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();
            match &msg.body {
                MessageBody::Init { node_id, .. } => {
                    eprintln!("Got Init message: {msg:?}");

                    // Initialize our node.
                    node_state.id = node_id.clone();

                    // Reply back with an init_ok.
                    send_init_reply(&mut node_state, msg);
                }
                MessageBody::Echo { .. } => {
                    eprintln!("Got Echo message: {msg:?}");

                    send_echo_reply(&mut node_state, msg);
                }
                _ => todo!(),
            };
        }
    }
}
