use crate::timestamp::TimeStamp;
use deku::prelude::*;
use num_enum::IntoPrimitive;

/// Used to communicate Server responses to Control-Client throughout TWAMP-Control protocol.
#[derive(Clone, Debug, Default, PartialEq, Copy, IntoPrimitive, DekuRead, DekuWrite)]
#[repr(u8)]
#[deku(type = "u8", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum Accept {
    /// Ok.
    #[default]
    Ok = 0,

    /// Failure, reason unspecified (catch-all).
    Failure = 1,

    /// Internal error.
    InternalError = 2,

    /// Some aspect of request is not supported.
    NotSupported = 3,

    /// Cannot perform request due to permanent resource limitations.
    PermanentResourceLimitation = 4,

    /// Cannot perform request due to temporary resource limitations.
    TemporaryResourceLimitation = 5,
}

/// Sent by Server to Control-Client after receiving a [Set-Up-Response](crate::set_up_response::SetUpResponse) command.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ServerStart {
    /// MBZ (Must Be Zero).
    pub mbz_start: [u8; 15],

    /// Indicates Server's willingness to continue. Possible values are [here](Accept).
    pub accept: Accept,

    /// Generated randomly. Unused in
    /// [unauthenticated mode](crate::security_mode::Mode::Unauthenticated).
    pub server_iv: [u8; 16],

    /// The time when the Server binary was executed.
    pub start_time: TimeStamp,

    /// MBZ (Must Be Zero).
    pub mbz_end: [u8; 8],
}

impl ServerStart {
    /// Create instance with provided accept value.
    pub fn new(accept: Accept) -> Self {
        ServerStart {
            mbz_start: [0; 15],
            accept,
            server_iv: [0; 16],
            start_time: TimeStamp::new(),
            mbz_end: [0; 8],
        }
    }
}

impl Default for ServerStart {
    fn default() -> Self {
        ServerStart {
            mbz_start: [0; 15],
            accept: Accept::Ok,
            server_iv: [0; 16],
            start_time: TimeStamp::new(),
            mbz_end: [0; 8],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn should_have_correct_bytes_of_struct() {
        assert_eq!(size_of::<ServerStart>(), 48);
    }

    #[test]
    fn should_serialize_to_correct_bytes() {
        let server_start = ServerStart::default();
        let encoded = server_start.to_bytes().unwrap();
        assert_eq!(encoded.len(), size_of::<ServerStart>());
    }

    #[test]
    fn should_deserialize_to_correct_struct() {
        let server_start = ServerStart::default();
        let encoded = server_start.to_bytes().unwrap();
        let (_rest, val) = ServerStart::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, server_start);
    }
}
