pub mod utils;

#[cfg(test)]
mod tests {
    use aurora::{InitBody, EchoBody, IdBody};
    use serde::de::DeserializeOwned;

    use super::utils::*;

    #[test]
    fn init_tests() {
        /* ------ Request ------ */
        let req = known_init_body();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, KNOWN_INIT_BODY);
        let data: InitBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, req);

        /* ------ Response ------ */
        let resp = known_init_ok_body();
        let json = serde_json::to_string(&resp).unwrap();
        assert_eq!(json, KNOWN_INIT_OK_BODY);
        let data: InitBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, resp);
    }

    #[test]
    fn echo_tests() {
        /* ------ Request ------ */
        let req = known_echo_body();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, KNOWN_ECHO_BODY);
        let data: EchoBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, req);

        /* ------ Response ------ */
        let resp = known_echo_ok_body();
        let json = serde_json::to_string(&resp).unwrap();
        assert_eq!(json, KNOWN_ECHO_OK_BODY);
        let data: EchoBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, resp);
    }

    #[test]
    fn id_tests() {
        /* ------ Request ------ */
        let req = known_id_body();
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, KNOWN_ID_BODY);
        let data: IdBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, req);

        /* ------ Response ------ */
        let resp = known_id_ok_body();
        let json = serde_json::to_string(&resp).unwrap();
        assert_eq!(json, KNOWN_ID_OK_BODY);
        let data: IdBody = serde_json::from_str(&json).unwrap();
        assert_eq!(data, resp);
    }

    #[test]
    fn no_mixed_signals() {
        fn valid_deserialization<T: DeserializeOwned + PartialEq>(s: &str, known: T) -> bool {
            serde_json::from_str::<T>(s)
                .map(|val| val == known)
                .unwrap_or_default()
        }
        let data = vec![
            KNOWN_INIT_BODY,
            KNOWN_INIT_OK_BODY,
            KNOWN_ECHO_BODY,
            KNOWN_ECHO_OK_BODY,
            KNOWN_ID_BODY,
            KNOWN_ID_OK_BODY,
        ];
        for datum in data {
            let mut count = 0;
            count += valid_deserialization::<InitBody>(datum, known_init_body()) as u8;
            count += valid_deserialization::<InitBody>(datum, known_init_ok_body()) as u8;
            count += valid_deserialization::<EchoBody>(datum, known_echo_body()) as u8;
            count += valid_deserialization::<EchoBody>(datum, known_echo_ok_body()) as u8;
            count += valid_deserialization::<IdBody>(datum, known_id_body()) as u8;
            count += valid_deserialization::<IdBody>(datum, known_id_ok_body()) as u8;
            assert_eq!(count, 1);
        }
    }
}
