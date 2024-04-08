#![allow(dead_code)]

use crate::security_mode::Mode;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// Sent by Control-Client to Server through TWAMP-Control after receiving
/// [Server Greeting](crate::server_greeting::ServerGreeting).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SetUpResponse {
    /// The [security mode](crate::security_mode::Mode) that `Control-Client` wishes to use.
    /// It **should** be a mode that the Server supports, which it had sent in
    /// [Server Greeting](crate::server_greeting::ServerGreeting::mode).
    pub mode: Mode,

    /// UTF-8 string up to 80 bytes, padded with zeros if shorter. Tells `Server` which shared
    /// secret the client wishes to use to authenticate or encrypt.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::UnAuthenticated) and
    /// acts as MBZ (Must Be Zero).
    #[serde(with = "BigArray")]
    pub key_id: [u8; 80],

    /// Concatenation of [challenge](crate::server_greeting::ServerGreeting::challenge), AES
    /// Session-Key and HMAC-SHA1 Session-Key.
    ///
    /// Unused in [unauthenticated mode](crate::security_mode::Mode::UnAuthenticated) and
    /// acts as MBZ (Must Be Zero).
    #[serde(with = "BigArray")]
    pub token: [u8; 64],

    /// Unused in [unauthenticated mode](crate::security_mode::Mode::UnAuthenticated) and
    /// acts as MBZ (Must Be Zero).
    pub client_iv: [u8; 16],
}

impl SetUpResponse {
    /// Create instance from supported mode, panics otherwise.
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::UnAuthenticated => SetUpResponse {
                mode,
                key_id: [0; 80],
                token: [0; 64],
                client_iv: [0; 16],
            },
            Mode::Abort => panic!("Mode 0, server don't wanna continue"),
            _ => panic!("Not supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use bincode::Options;

    use crate::{security_mode::Mode, set_up_response::SetUpResponse};

    #[test]
    fn should_have_correct_size() {
        assert_eq!(size_of::<SetUpResponse>(), 164)
    }

    #[test]
    fn should_serialize_correctly() {
        let set_up_response = SetUpResponse::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&set_up_response)
            .unwrap();
        assert_eq!(encoded.len(), 164)
    }

    #[test]
    fn should_deserialize_to_struct() {
        let set_up_response = SetUpResponse::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&set_up_response)
            .unwrap();
        let decoded: SetUpResponse = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .deserialize(&encoded)
            .unwrap();
        assert_eq!(decoded, set_up_response)
    }
}
