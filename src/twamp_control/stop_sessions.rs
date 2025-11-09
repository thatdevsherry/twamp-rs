use super::{accept::Accept, command_number::CommandNumber};
use deku::prelude::*;

/// Server Greeting sent by `Server` to `Control-Client` after `Control-Client` opens up a TCP
/// connection.
///
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct StopSessions {
    #[deku(assert_eq = "CommandNumber::StopSessions")]
    command_number: CommandNumber,
    accept: Accept,
    #[deku(assert_eq = "0u16")]
    mbz: u16,
    hmac: [u8; 16],
}

impl StopSessions {
    pub fn new(accept: Accept) -> Self {
        StopSessions {
            command_number: CommandNumber::StopSessions,
            accept,
            mbz: 0,
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const STOP_SESSIONS_LENGTH_IN_BYTES: usize = 20;

    #[test]
    fn command_number_is_correct() {
        let stop_sessions = StopSessions::new(Accept::Ok);
        assert_eq!(stop_sessions.command_number, CommandNumber::StopSessions);
    }

    #[test]
    fn mbz_is_zero() {
        let stop_sessions = StopSessions::new(Accept::Ok);
        assert_eq!(stop_sessions.mbz, 0);
    }

    #[test]
    fn serialize_to_bytes() {
        let stop_sessions = StopSessions::new(Accept::Ok).to_bytes().unwrap();
        assert_eq!(stop_sessions.len(), STOP_SESSIONS_LENGTH_IN_BYTES);
    }

    #[test]
    #[ignore]
    fn deserialize_to_struct() {
        let stop_sessions_as_bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let (_rest, stop_sessions) =
            StopSessions::from_bytes((&stop_sessions_as_bytes, 0)).unwrap();
        assert_eq!(stop_sessions.command_number, CommandNumber::StopSessions);
        assert_eq!(stop_sessions.accept, Accept::Ok);
        assert_eq!(stop_sessions.mbz, 0u16);
        assert_eq!(stop_sessions.hmac, [0u8; 16]);
    }
}
