use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[repr(u8)]
enum Accept {
    #[default]
    Ok = 0u8,
    Failure = 1u8,
    InternalError = 2u8,
    NotSupported = 3u8,
    PermanentResourceLimitation = 4u8,
    TemporaryResourceLimitation = 5u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
struct TimeStamp {
    integer_part_of_seconds: [u8; 32],
    fractional_part_of_seconds: [u8; 32],
}

impl TimeStamp {
    pub fn new() -> Self {
        TimeStamp {
            integer_part_of_seconds: [0; 32],
            fractional_part_of_seconds: [0; 32],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ServerStart {
    mbz_start: [u8; 15],
    accept: u8,
    server_iv: [u8; 16],
    start_time: TimeStamp,
    mbz_end: [u8; 8],
}

impl ServerStart {
    pub fn new() -> Self {
        ServerStart {
            mbz_start: [0; 15],
            accept: (Accept::Ok) as u8,
            server_iv: [0; 16],
            start_time: TimeStamp::new(),
            mbz_end: [0; 8],
        }
    }
}
