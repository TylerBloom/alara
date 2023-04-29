use aurora::{EchoBody, InitBody, Message, MessageBody, MessageId, IdBody};
use const_format::formatcp;

/* ------ Init ------ */
pub const KNOWN_INIT_BODY: &str =
    r#"{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]}"#;
pub const KNOWN_INIT_OK_BODY: &str = r#"{"type":"init_ok","msg_id":0,"in_reply_to":1}"#;

pub fn known_init_body() -> InitBody {
    InitBody::Init {
        id: Some(MessageId(1)),
        node_id: String::from("n3"),
        node_ids: vec![String::from("n1"), String::from("n2"), String::from("n3")],
    }
}

pub fn known_init_ok_body() -> InitBody {
    InitBody::InitOk {
        id: MessageId(0),
        in_reply_to: Some(MessageId(1)),
    }
}

/* ------ Echo ------ */
pub const KNOWN_ECHO_BODY: &str = r#"{"type":"echo","msg_id":1,"echo":"Please echo 35"}"#;
pub const KNOWN_ECHO_OK_BODY: &str =
    r#"{"type":"echo_ok","echo":"Please echo 35","msg_id":1,"in_reply_to":1}"#;

pub fn known_echo_body() -> EchoBody {
    EchoBody::Echo {
        id: MessageId(1),
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
    IdBody::GenerateOk { id: 123.to_string() }
}

/* ------ Messages ------ */
const CLIENT_ID: &str = "c1";
const NODE_ID: &str = "n1";
const REQUEST_BASE: &str = formatcp!(r#"{{"src":"{CLIENT_ID}","dest":"{NODE_ID}","body":"#);
const RESPONSE_BASE: &str = formatcp!(r#"{{"src":"{NODE_ID}","dest":"{CLIENT_ID}","body":"#);
/*
{"src":"c1","dest":"n1","body":{"type":"init","msg_id":1,"node_id":"n3","node_ids":["n1","n2","n3"]}}
{"src":"c1","dest":"n1","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}
*/

pub const KNOWN_INIT_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_INIT_BODY}}}");
pub const KNOWN_INIT_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_INIT_OK_BODY}}}");
pub const KNOWN_ECHO_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_ECHO_BODY}}}");
pub const KNOWN_ECHO_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_ECHO_OK_BODY}}}");
pub const KNOWN_ID_MSG: &str = formatcp!("{REQUEST_BASE}{KNOWN_ID_BODY}}}");
pub const KNOWN_ID_OK_MSG: &str = formatcp!("{RESPONSE_BASE}{KNOWN_ID_OK_BODY}}}");

pub fn known_request<B: MessageBody>(body: B) -> Message<B> {
    Message {
        src: String::from(CLIENT_ID),
        dest: String::from(NODE_ID),
        body,
    }
}

pub fn known_response<B: MessageBody>(body: B) -> Message<B> {
    let Message { src, dest, body } = known_request(body);
    Message {
        src: dest,
        dest: src,
        body,
    }
}
