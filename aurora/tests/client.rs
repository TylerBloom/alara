#[cfg(test)]
mod tests {
    use aurora::{Message, MessageBody, MessageId, Node, Client};
    use serde::{Deserialize, Serialize};
    use tokio::sync::mpsc::UnboundedSender;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    struct DummyBody;

    impl MessageBody for DummyBody {}

    struct DummyNode;

    impl Node for DummyNode {
        type Body = DummyBody;

        fn init(
            _: UnboundedSender<Message<Self::Body>>,
            _: String,
            _: Vec<String>,
        ) -> Self {
            Self
        }

        fn next_id(&mut self) -> MessageId {
            MessageId(0)
        }

        fn handle_msg(
            &mut self,
            _: Message<Self::Body>,
        ) -> anyhow::Result<Option<Message<Self::Body>>> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn dropped_sender() {
        let (mut client, _node): (_, DummyNode) = Client::new().await;
        let has_dropped = tokio::select! {
            _ = client.next_msg() => false,
            _ = tokio::time::sleep(std::time::Duration::from_millis(5)) => true,
        };
        assert!(has_dropped)
    }
}
