use aurora::*;
use serde::{Deserialize, Serialize};

fn main() {
    let (mut client, mut node): (_, EchoNode) = Client::new();
    while let Some(msg) = client.next_msg() {
        if let Some(resp) = node
            .handle_msg(&mut client, msg)
            .expect("handling echo requests should always succeed")
        {
            client.send_msg(resp);
        }
    }
}

struct EchoNode {
    counter: usize,
}

impl Node for EchoNode {
    type Body = EchoRequest;

    fn init(_msg: &Message<InitRequest>) -> Self {
        Self { counter: 0 }
    }

    fn next_id(&mut self) -> MessageId {
        let id = MessageId(self.counter);
        self.counter += 1;
        id
    }

    fn handle_msg(
        &mut self,
        _client: &mut Client<Self>,
        msg: Message<Self::Body>,
    ) -> anyhow::Result<Option<Message<Self::Body>>> {
        match &msg.body {
            MessageBody::Main(_) => Ok(Some(msg.into_response(self.next_id()))),
            _ => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename = "echo")]
struct EchoRequest {
    #[serde(rename = "msg_id")]
    id: MessageId,
    echo: String,
}

impl RequestBody for EchoRequest {
    type Response = EchoResponse;

    fn into_response(self, _id: MessageId) -> Self::Response {
        Self::Response {
            in_reply_to: self.id,
            echo: self.echo,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename = "echo_ok")]
struct EchoResponse {
    echo: String,
    in_reply_to: MessageId,
}

impl ResponseBody for EchoResponse {
    type Request = EchoRequest;
}
