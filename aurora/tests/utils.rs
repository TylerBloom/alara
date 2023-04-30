use aurora::{BroadcastBody, EchoBody, IdBody, InitBody, Message, MessageBody, MessageId};
use const_format::formatcp;

/* ------ Init ------ */
pub const KNOWN_INIT_BODY: &str =
    r#"{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]}"#;
pub const KNOWN_INIT_OK_BODY: &str = r#"{"type":"init_ok","msg_id":0,"in_reply_to":1}"#;

pub fn known_init_body() -> InitBody {
    InitBody::Init {
        msg_id: Some(MessageId(1)),
        node_id: String::from("n3"),
        node_ids: vec![String::from("n1"), String::from("n2"), String::from("n3")],
    }
}

pub fn known_init_ok_body() -> InitBody {
    InitBody::InitOk {
        msg_id: MessageId(0),
        in_reply_to: Some(MessageId(1)),
    }
}

/* ------ Echo ------ */
pub const KNOWN_ECHO_BODY: &str = r#"{"type":"echo","msg_id":1,"echo":"Please echo 35"}"#;
pub const KNOWN_ECHO_OK_BODY: &str =
    r#"{"type":"echo_ok","echo":"Please echo 35","msg_id":1,"in_reply_to":1}"#;

pub fn known_echo_body() -> EchoBody {
    EchoBody::Echo {
        msg_id: MessageId(1),
        echo: String::from("Please echo 35"),
    }
}

pub fn known_echo_ok_body() -> EchoBody {
    EchoBody::EchoOk {
        echo: String::from("Please echo 35"),
        msg_id: MessageId(1),
        in_reply_to: MessageId(1),
    }
}

/* ------ Ids ------ */
pub const KNOWN_ID_BODY: &str = r#"{"type":"generate"}"#;
pub const KNOWN_ID_OK_BODY: &str = r#"{"type":"generate_ok","id":"123"}"#;

pub fn known_id_body() -> IdBody {
    IdBody::Generate
}

pub fn known_id_ok_body() -> IdBody {
    IdBody::GenerateOk {
        id: 123.to_string(),
    }
}

/* ------ Broadcast ------ */
pub const KNOWN_BROADCAST_BODY: &str = r#"{"type":"broadcast","msg_id":1,"message":1000}"#;
pub const KNOWN_BROADCAST_OK_BODY: &str = r#"{"type":"broadcast_ok","msg_id":2,"in_reply_to":1}"#;
pub const KNOWN_READ_BODY: &str = r#"{"type":"read","msg_id":1}"#;
pub const KNOWN_READ_OK_BODY: &str =
    r#"{"type":"read_ok","msg_id":2,"in_reply_to":1,"messages":[25,72,1,8]}"#;
pub const KNOWN_TOPOLOGY_BODY: &str =
    r#"{"type":"topology","msg_id":1,"topology":{"n1":["n2","n3"],"n2":["n1"],"n3":["n1"]}}"#;
pub const KNOWN_TOPOLOGY_OK_BODY: &str = r#"{"type":"topology_ok","msg_id":2,"in_reply_to":1}"#;

pub fn known_broadcast_body() -> BroadcastBody {
    BroadcastBody::Broadcast {
        msg_id: MessageId(1),
        message: 1000,
    }
}

pub fn known_broadcast_ok_body() -> BroadcastBody {
    BroadcastBody::BroadcastOk {
        msg_id: MessageId(2),
        in_reply_to: MessageId(1),
    }
}

pub fn known_read_body() -> BroadcastBody {
    BroadcastBody::Read {
        msg_id: MessageId(1),
    }
}

pub fn known_read_ok_body() -> BroadcastBody {
    BroadcastBody::ReadOk {
        msg_id: MessageId(2),
        in_reply_to: MessageId(1),
        messages: [1, 8, 72, 25].into_iter().collect(),
    }
}

pub fn known_topology_body() -> BroadcastBody {
    let topology = [
        (
            "n1".into(),
            ["n2".into(), "n3".into()].into_iter().collect(),
        ),
        ("n2".into(), ["n1".into()].into_iter().collect()),
        ("n3".into(), ["n1".into()].into_iter().collect()),
    ]
    .into_iter()
    .collect();
    BroadcastBody::Topology {
        msg_id: MessageId(1),
        topology,
    }
}

pub fn known_topology_ok_body() -> BroadcastBody {
    BroadcastBody::TopologyOk {
        msg_id: MessageId(2),
        in_reply_to: MessageId(1),
    }
}

/* ------ Messages ------ */
const CLIENT_ID: &str = "c1";
const NODE_ID: &str = "n1";
const REQUEST_BASE: &str = formatcp!(r#"{{"src":"{CLIENT_ID}","dest":"{NODE_ID}","body":"#);
const RESPONSE_BASE: &str = formatcp!(r#"{{"src":"{NODE_ID}","dest":"{CLIENT_ID}","body":"#);
/*
{"src":"c1","dest":"n1","body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]}}
{"src":"c1","dest":"n1","body":{"type":"broadcast","message":1000}}
{"src":"c1","dest":"n1","body":{"type":"topology","topology":{}}}
{"src":"c1","dest":"n1","body":{"type":"read"}}
*/

pub const KNOWN_INIT_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_INIT_BODY}}}");
pub const KNOWN_INIT_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_INIT_OK_BODY}}}");

pub const KNOWN_ECHO_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_ECHO_BODY}}}");
pub const KNOWN_ECHO_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_ECHO_OK_BODY}}}");

pub const KNOWN_ID_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_ID_BODY}}}");
pub const KNOWN_ID_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_ID_OK_BODY}}}");

pub const KNOWN_BROADCAST_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_BROADCAST_BODY}}}");
pub const KNOWN_BROADCAST_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_BROADCAST_OK_BODY}}}");
pub const KNOWN_READ_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_READ_BODY}}}");
pub const KNOWN_READ_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_READ_OK_BODY}}}");
pub const KNOWN_TOPOLOGY_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_TOPOLOGY_BODY}}}");
pub const KNOWN_TOPOLOGY_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_TOPOLOGY_OK_BODY}}}");

pub fn known_request<B: MessageBody>(body: B) -> Message<B> {
    Message {
        src: String::from(CLIENT_ID),
        dest: String::from(NODE_ID),
        body,
    }
}

pub fn known_response<B: MessageBody>(body: B) -> Message<B> {
    let mut msg = known_request(body);
    msg.into_response(|_| ());
    msg
}
