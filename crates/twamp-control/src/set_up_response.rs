#![allow(dead_code)]

use crate::security_mode::Mode;
use deku::prelude::*;

/// Sent by Control-Client to Server through TWAMP-Control after receiving
/// [Server Greeting](crate::server_greeting::ServerGreeting).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct SetUpResponse {
    /// The [security mode](crate::security_mode::Mode) that `Control-Client` wishes to use.
    /// It **should** be a mode that the Server supports, which it had sent in
    /// [Server Greeting](crate::server_greeting::ServerGreeting::mode).
    pub mode: Mode,

    /// UTF-8 string up to 80 bytes, padded with zeros if shorter. Tells `Server` which shared
    /// secret the client wishes to use to authenticate or encrypt.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    pub key_id: [u8; 80],

    /// Concatenation of [challenge](crate::server_greeting::ServerGreeting::challenge), AES
    /// Session-Key and HMAC-SHA1 Session-Key.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    pub token: [u8; 64],

    /// Unused in [unauthenticated mode](crate::security_mode::Mode::Unauthenticated) and
    /// acts as MBZ (Must Be Zero).
    pub client_iv: [u8; 16],
}

impl SetUpResponse {
    /// Create instance from supported mode, panics otherwise.
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Unauthenticated => SetUpResponse {
                mode,
                key_id: [0; 80],
                token: [0; 64],
                client_iv: [0; 16],
            },
            Mode::Reserved => panic!("Mode 0, server don't wanna continue"),
            _ => panic!("Not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn should_have_correct_size() {
        assert_eq!(size_of::<SetUpResponse>(), 164)
    }

    #[test]
    fn should_serialize_correctly() {
        let set_up_response = SetUpResponse::new(Mode::Unauthenticated);
        let encoded = set_up_response.to_bytes().unwrap();
        assert_eq!(encoded.len(), 164)
    }

    #[test]
    fn should_deserialize_to_struct() {
        let set_up_response = SetUpResponse::new(Mode::Unauthenticated);
        let encoded = set_up_response.to_bytes().unwrap();
        let (_rest, val) = SetUpResponse::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, set_up_response)
    }
}
