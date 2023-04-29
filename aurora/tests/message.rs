pub mod utils;

#[cfg(test)]
mod tests {
    use aurora::{Message, InitBody, EchoBody};

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
}
