use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageBody {
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
    Topology {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
    },
    Broadcast {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        message: usize,
    },
    BroadcastOk {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
    },
    Read {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
    },
    ReadOk {
        msg_id: Option<usize>,
        in_reply_to: Option<usize>,
        messages: Vec<usize>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}
