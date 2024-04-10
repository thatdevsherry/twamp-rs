use crate::server_start::Accept;
use deku::prelude::*;

/// Response for a Request-TW-Session command.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AcceptSession {
    /// Represents Server's willingness to continue or reject.
    pub accept: Accept,

    /// MBZ (Must Be Zero).
    pub mbz_first: u8,

    /// Either the port that was present in Request-TW-Session or an alternative port in case the
    /// requested port by Control-Client is not available.
    pub port: u16,

    pub sid: [u8; 16],

    /// MBZ (Must Be Zero).
    pub mbz_second: [u8; 12],

    pub hmac: [u8; 16],
}

impl AcceptSession {
    /// Construct from an Accept value and port. It sets sid and hmac as zeros.
    pub fn new(accept: Accept, port: u16) -> Self {
        AcceptSession {
            accept,
            mbz_first: 0,
            port,
            sid: [0; 16],
            mbz_second: [0; 12],
            hmac: [0; 16],
        }
    }
}

impl Default for AcceptSession {
    fn default() -> Self {
        AcceptSession {
            accept: Accept::Ok,
            mbz_first: 0,
            port: 0,
            sid: [0; 16],
            mbz_second: [0; 12],
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn should_have_correct_size_of_struct() {
        assert_eq!(size_of::<AcceptSession>(), 48);
    }

    #[test]
    fn should_serialize_into_correct_length_of_bytes() {
        let accept_session = AcceptSession::default();
        let encoded = accept_session.to_bytes().unwrap();
        assert_eq!(encoded.len(), size_of::<AcceptSession>());
    }

    #[test]
    fn should_deserialize_into_correct_length_of_bytes() {
        let accept_session = AcceptSession::default();
        let encoded = accept_session.to_bytes().unwrap();
        let (_rest, val) = AcceptSession::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, accept_session);
    }
}
