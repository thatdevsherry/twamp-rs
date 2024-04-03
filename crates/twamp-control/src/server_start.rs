use serde::{Deserialize, Serialize};

/// Used to communicate Server responses to Control-Client throughout TWAMP-Control protocol.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
pub enum Accept {
    /// Ok.
    #[default]
    Ok = 0u8,

    /// Failure, reason unspecified (catch-all).
    Failure = 1u8,

    /// Internal error.
    InternalError = 2u8,

    /// Some aspect of request is not supported.
    NotSupported = 3u8,

    /// Cannot perform request due to permanent resource limitations.
    PermanentResourceLimitation = 4u8,

    /// Cannot perform request due to temporary resource limitations.
    TemporaryResourceLimitation = 5u8,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimeStamp {
    pub integer_part_of_seconds: [u8; 32],
    pub fractional_part_of_seconds: [u8; 32],
}

impl TimeStamp {
    pub fn new() -> Self {
        TimeStamp {
            integer_part_of_seconds: [0; 32],
            fractional_part_of_seconds: [0; 32],
        }
    }
}

/// Sent by Server to Control-Client after receiving a [Set-Up-Response](crate::set_up_response::SetUpResponse) command.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerStart {
    /// MBZ (Must Be Zero).
    pub mbz_start: [u8; 15],

    /// Indicates Server's willingness to continue. Possible values are [here](Accept).
    pub accept: u8,

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
            accept: (accept) as u8,
            server_iv: [0; 16],
            start_time: TimeStamp::new(),
            mbz_end: [0; 8],
        }
    }
}
