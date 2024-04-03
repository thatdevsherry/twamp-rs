use serde::{Deserialize, Serialize};

/// Security Mode. See details in
/// [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[repr(u32)]
pub enum Mode {
    Abort = 0u32,
    #[default]
    UnAuthenticated = 1u32,
    Authenticated = 2u32,
    Encrypted = 4u32,
}
