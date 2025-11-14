use super::SecurityMode;
use anyhow::Result;
use deku::prelude::*;

/// Sent by Control-Client to Server through TWAMP-Control after receiving
/// [Server Greeting](crate::server_greeting::ServerGreeting).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct SetUpResponse {
    /// The [security mode](crate::security_mode::Mode) that `Control-Client` wishes to use.
    /// It **should** be a mode that the Server supports, which it had sent in
    /// [Server Greeting](crate::server_greeting::ServerGreeting).
    mode: SecurityMode,

    /// UTF-8 string up to 80 bytes, padded with zeros if shorter. Tells `Server` which shared
    /// secret the client wishes to use to authenticate or encrypt.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    key_id: [u8; 80],

    /// Concatenation of [challenge](crate::server_greeting::ServerGreeting::challenge), AES
    /// Session-Key and HMAC-SHA1 Session-Key.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    token: [u8; 64],

    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    client_iv: [u8; 16],
}

impl SetUpResponse {
    /// Attempt to create Set-Up-Response with provided mode.
    ///
    /// Errors if the provided mode is not supported by `twamp-rs`.
    pub fn new(mode: SecurityMode) -> Result<Self, String> {
        match mode {
            SecurityMode::Reserved | SecurityMode::Unauthenticated => Ok(SetUpResponse {
                mode,
                key_id: [0; 80],
                token: [0; 64],
                client_iv: [0; 16],
            }),
            _ => Err(format!(
                "twamp-rs ONLY supports unauthenticated mode, mode provided is {:?}",
                mode
            )
            .to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_UP_RESPONSE_LENGTH_IN_BYTES: usize = 164;

    #[test]
    fn unused_key_id_in_unauth_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Unauthenticated)
            .expect("should have created set_up_response.");
        assert_eq!(set_up_response.key_id.iter().fold(0, |acc, v| acc + v), 0);
    }

    #[test]
    fn unused_token_in_unauth_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Unauthenticated)
            .expect("should have created set_up_response.");
        assert_eq!(set_up_response.token.iter().fold(0, |acc, v| acc + v), 0);
    }

    #[test]
    fn unused_client_iv_in_unauth_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Unauthenticated)
            .expect("should have created set_up_response.");
        assert_eq!(
            set_up_response.client_iv.iter().fold(0, |acc, v| acc + v),
            0
        );
    }

    #[test]
    fn unused_key_id_in_reserved_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Reserved)
            .expect("should have created set_up_response.");
        assert_eq!(set_up_response.key_id.iter().fold(0, |acc, v| acc + v), 0);
    }

    #[test]
    fn unused_token_in_reserved_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Reserved)
            .expect("should have created set_up_response.");
        assert_eq!(set_up_response.token.iter().fold(0, |acc, v| acc + v), 0);
    }

    #[test]
    fn unused_client_iv_in_reserved_mode() {
        let set_up_response = SetUpResponse::new(SecurityMode::Reserved)
            .expect("should have created set_up_response.");
        assert_eq!(
            set_up_response.client_iv.iter().fold(0, |acc, v| acc + v),
            0
        );
    }

    /// Unsupported mode by twamp-rs.
    #[test]
    #[should_panic]
    fn panic_on_mode_auth() {
        SetUpResponse::new(SecurityMode::Authenticated)
            .expect("should have created set_up_response.");
    }

    /// Unsupported mode by twamp-rs.
    #[test]
    #[should_panic]
    fn panic_on_mode_encrypted() {
        SetUpResponse::new(SecurityMode::Encrypted).expect("should have created set_up_response.");
    }

    /// Unsupported mode by twamp-rs.
    #[test]
    #[should_panic]
    fn panic_on_mode_mixed_security() {
        SetUpResponse::new(SecurityMode::EncryptedControlUnauthTest)
            .expect("should have created set_up_response.");
    }

    #[test]
    fn serialize_to_correct_length_of_bytes() {
        let set_up_response = SetUpResponse::new(SecurityMode::Unauthenticated)
            .expect("should have created set_up_response.");
        let encoded = set_up_response.to_bytes().unwrap();
        assert_eq!(encoded.len(), SET_UP_RESPONSE_LENGTH_IN_BYTES)
    }

    #[test]
    fn deserialize_to_struct() {
        let set_up_response = SetUpResponse::new(SecurityMode::Unauthenticated)
            .expect("should have created set_up_response.");
        let encoded = set_up_response.to_bytes().unwrap();
        let (_rest, val) = SetUpResponse::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, set_up_response)
    }
}
