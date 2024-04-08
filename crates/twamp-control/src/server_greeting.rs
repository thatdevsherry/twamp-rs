use std::mem::size_of;

use rand::random;

use crate::security_mode::Mode;
use bincode::Options;
use serde::{Deserialize, Serialize};

/// Server Greeting sent by `Server` to `Control-Client` after it opens up a TCP connection.
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerGreeting {
    /// Same semantics as MBZ (Must Be Zero).
    pub unused: [u8; 12],

    /// Security mode(s) that the Server supports.
    pub mode: Mode,

    /// Random seq of bytes.
    ///
    /// Set regardless of the client's security mode capability.
    pub challenge: [u8; 16],

    /// MUST be generated pseudo-randomly.
    ///
    /// Set regardless of the client's security mode capability.
    pub salt: [u8; 16],

    /// TWAMP sets default MAX value SHOULD be 32768. It can be increased if computing
    /// power can handle.
    ///
    /// Set regardless of the client's security mode capability.
    pub count: [u8; 4],

    /// Must Be Zero.
    pub mbz: [u8; 12],
}

impl ServerGreeting {
    /// Create instance with supported modes.
    pub fn new(mode: Mode) -> Self {
        ServerGreeting {
            unused: [0; 12],
            mode,
            challenge: [random::<u8>(); 16],
            salt: [random::<u8>(); 16],
            count: *b"1024",
            mbz: [0; 12],
        }
    }
}

impl Default for ServerGreeting {
    fn default() -> Self {
        ServerGreeting {
            unused: [0; 12],
            mode: Mode::UnAuthenticated,
            challenge: [random::<u8>(); 16],
            salt: [random::<u8>(); 16],
            count: *b"1024",
            mbz: [0; 12],
        }
    }
}

impl TryFrom<ServerGreeting> for Vec<u8> {
    type Error = &'static str;

    fn try_from(value: ServerGreeting) -> Result<Self, Self::Error> {
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&value)
            .expect("should have converted to vec.");
        return Ok(encoded);
    }
}

impl TryFrom<[u8; size_of::<ServerGreeting>()]> for ServerGreeting {
    type Error = &'static str;
    fn try_from(value: [u8; size_of::<ServerGreeting>()]) -> Result<Self, Self::Error> {
        let decoded: ServerGreeting = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&value)
            .expect("should have converted to struct.");
        return Ok(decoded);
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use bincode::Options;

    use super::*;

    #[test]
    fn should_create_server_greeting_with_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        assert_eq!(server_greeting.mode, Mode::UnAuthenticated);
    }

    #[test]
    fn should_have_correct_size_of_struct() {
        assert_eq!(size_of::<ServerGreeting>(), 64);
    }

    #[test]
    fn should_serialize_into_correct_length_of_bytes() {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_greeting)
            .unwrap();
        assert_eq!(encoded.len(), size_of::<ServerGreeting>());
    }

    #[test]
    fn should_deserialize_into_correct_struct() {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_greeting)
            .unwrap();
        let decoded: ServerGreeting = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian()
            .deserialize(&encoded)
            .unwrap();
        assert_eq!(decoded, server_greeting);
    }
}
