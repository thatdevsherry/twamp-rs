#![allow(dead_code)]

use rand::random;

use crate::security_mode::Mode;
use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ServerGreeting {
    unused: [u8; 12],
    pub mode: u32,
    challenge: [u8; 16],
    salt: [u8; 16],
    count: [u8; 4],
    mbz: [u8; 12],
}

impl ServerGreeting {
    pub fn new(mode: Mode) -> Self {
        ServerGreeting {
            unused: [0; 12],
            mode: (mode as u32),
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
    fn should_create_server_greeting_with_mode_abort() {
        let server_greeting = ServerGreeting::new(Mode::Abort);
        assert_eq!(server_greeting.mode, [0, 0, 0, 0]);
    }

    #[test]
    fn should_create_server_greeting_with_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(Mode::UnAuthenticated);
        assert_eq!(server_greeting.mode, [0, 0, 0, 1]);
    }

    #[test]
    fn should_create_server_greeting_with_mode_authenticated() {
        let server_greeting = ServerGreeting::new(Mode::Authenticated);
        assert_eq!(server_greeting.mode, [0, 0, 0, 2]);
    }

    #[test]
    fn should_create_server_greeting_with_mode_encrypted() {
        let server_greeting = ServerGreeting::new(Mode::Encrypted);
        assert_eq!(server_greeting.mode, [0, 0, 0, 4]);
    }
}
