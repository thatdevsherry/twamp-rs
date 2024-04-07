use serde::{Deserialize, Serialize};

/// Security Mode. See details in
/// [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Copy)]
#[repr(u32)]
#[serde(into = "u32")]
pub enum Mode {
    Abort = 0,
    #[default]
    UnAuthenticated = 1,
    Authenticated = 2,
    Encrypted = 4,
}

impl From<Mode> for u32 {
    fn from(value: Mode) -> Self {
        value as u32
    }
}
