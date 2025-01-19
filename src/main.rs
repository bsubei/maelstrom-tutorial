use std::io;
use std::io::BufRead;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
}

#[derive(Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let msg: Message = serde_json::from_str(&line).unwrap();
            eprintln!("Received message: {}", serde_json::to_string(&msg).unwrap());
        }
    }
}
