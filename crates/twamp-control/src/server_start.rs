use serde::{Deserialize, Serialize};

/// Used to communicate Server responses to Control-Client throughout TWAMP-Control protocol.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Copy)]
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

impl From<Accept> for u8 {
    fn from(value: Accept) -> Self {
        value as u8
    }
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
