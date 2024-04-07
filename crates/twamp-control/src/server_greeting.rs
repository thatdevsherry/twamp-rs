use rand::random;

use crate::security_mode::Mode;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_server_greeting_with_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        assert_eq!(server_greeting.mode, Mode::UnAuthenticated);
    }
}
