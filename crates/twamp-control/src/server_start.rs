use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

use crate::timestamp::TimeStamp;

/// Used to communicate Server responses to Control-Client throughout TWAMP-Control protocol.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Copy, IntoPrimitive)]
#[repr(u8)]
#[serde(into = "u8")]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerStart {
    /// MBZ (Must Be Zero).
    pub mbz_start: [u8; 15],

    /// Indicates Server's willingness to continue. Possible values are [here](Accept).
    pub accept: Accept,

    /// Generated randomly. Unused in
    /// [unauthenticated mode](crate::security_mode::Mode::UnAuthenticated).
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
    use std::mem::size_of;

    use bincode::Options;

    use super::ServerStart;

    #[test]
    fn should_have_correct_bytes_of_struct() {
        assert_eq!(size_of::<ServerStart>(), 48);
    }

    #[test]
    fn should_serialize_to_correct_bytes() {
        let server_start = ServerStart::default();
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_start)
            .unwrap();
        assert_eq!(encoded.len(), size_of::<ServerStart>());
    }

    #[test]
    fn should_deserialize_to_correct_struct() {
        let server_start = ServerStart::default();
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&server_start)
            .unwrap();
        let decoded: ServerStart = bincode::DefaultOptions::new()
            .with_big_endian()
            .allow_trailing_bytes()
            .deserialize(&encoded)
            .unwrap();
        assert_eq!(decoded, server_start);
    }
}
