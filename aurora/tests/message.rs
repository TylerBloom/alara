pub mod utils;

#[cfg(test)]
mod tests {
    use aurora::{BroadcastBody, EchoBody, InitBody, Message, OrInit, IdBody};

    use super::utils::*;

    #[test]
    fn init_request_tests() {
        let msg = known_request(known_init_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_INIT_MSG);
        let data: Message<InitBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn init_reponse_tests() {
        let msg = known_response(known_init_ok_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_INIT_OK_MSG);
        let data: Message<InitBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn echo_request_tests() {
        let msg = known_request(known_echo_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_ECHO_MSG);
        let data: Message<EchoBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn echo_reponse_tests() {
        let msg = known_response(known_echo_ok_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_ECHO_OK_MSG);
        let data: Message<EchoBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn broadcast_request_tests() {
        let msg = known_request(known_broadcast_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_BROADCAST_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn broadcast_reponse_tests() {
        let msg = known_response(known_broadcast_ok_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_BROADCAST_OK_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn read_request_tests() {
        let msg = known_request(known_read_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_READ_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn read_reponse_tests() {
        let msg = known_response(known_read_ok_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_READ_OK_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn topology_request_tests() {
        let msg = known_request(known_topology_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_TOPOLOGY_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    #[test]
    fn topology_reponse_tests() {
        let msg = known_response(known_topology_ok_body());
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, KNOWN_TOPOLOGY_OK_MSG);
        let data: Message<BroadcastBody> = serde_json::from_str(&json).unwrap();
        assert_eq!(data, msg);
    }

    /* ------ OrInit ------ */
    #[test]
    fn no_mixed_signals_with_or_init() {
        assert_eq!(
            serde_json::from_str::<OrInit<EchoBody>>(KNOWN_ECHO_MSG).unwrap(),
            OrInit::Main(known_request(known_echo_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<EchoBody>>(KNOWN_ECHO_OK_MSG).unwrap(),
            OrInit::Main(known_response(known_echo_ok_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<IdBody>>(KNOWN_ID_MSG).unwrap(),
            OrInit::Main(known_request(known_id_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<IdBody>>(KNOWN_ID_OK_MSG).unwrap(),
            OrInit::Main(known_response(known_id_ok_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_BROADCAST_MSG).unwrap(),
            OrInit::Main(known_request(known_broadcast_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_BROADCAST_OK_MSG).unwrap(),
            OrInit::Main(known_response(known_broadcast_ok_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_READ_MSG).unwrap(),
            OrInit::Main(known_request(known_read_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_READ_OK_MSG).unwrap(),
            OrInit::Main(known_response(known_read_ok_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_TOPOLOGY_MSG).unwrap(),
            OrInit::Main(known_request(known_topology_body()))
        );
        assert_eq!(
            serde_json::from_str::<OrInit<BroadcastBody>>(KNOWN_TOPOLOGY_OK_MSG).unwrap(),
            OrInit::Main(known_response(known_topology_ok_body()))
        );
    }
}
