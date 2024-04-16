use crate::command_number::CommandNumber;
use deku::prelude::*;

/// Server Greeting sent by `Server` to `Control-Client` after `Control-Client` opens up a TCP
/// connection.
///
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct StartSessions {
    #[deku(assert_eq = "CommandNumber::StartSessions")]
    command_number: CommandNumber,
    #[deku(assert_eq = "[0u8; 15]")]
    mbz: [u8; 15],
    hmac: [u8; 16],
}

impl StartSessions {
    pub fn new() -> Self {
        StartSessions {
            command_number: CommandNumber::StartSessions,
            mbz: [0; 15],
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StartSessions;
    use crate::command_number::CommandNumber;
    use deku::{DekuContainerRead, DekuContainerWrite};

    const START_SESSIONS_LENGTH_IN_BYTES: usize = 32;

    #[test]
    fn command_number_is_correct() {
        let start_sessions = StartSessions::new();
        assert_eq!(start_sessions.command_number, CommandNumber::StartSessions);
    }

    #[test]
    fn mbz_is_zero() {
        let start_sessions = StartSessions::new();
        assert_eq!(start_sessions.mbz, [0; 15]);
    }

    #[test]
    #[ignore]
    fn hmac_is_assigned() {
        todo!();
    }

    #[test]
    fn serialize_to_bytes() {
        let start_sessions = StartSessions::new().to_bytes().unwrap();
        assert_eq!(start_sessions.len(), START_SESSIONS_LENGTH_IN_BYTES);
    }

    #[test]
    fn deserialize_to_struct() {
        let start_sessions_as_bytes = [
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let (_rest, start_sessions) =
            StartSessions::from_bytes((&start_sessions_as_bytes, 0)).unwrap();
        assert_eq!(start_sessions.command_number, CommandNumber::StartSessions);
        assert_eq!(start_sessions.mbz, [0u8; 15]);
        assert_eq!(start_sessions.hmac, [0u8; 16]);
    }
}
