use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

/// Security Mode. See details in
/// [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Copy, IntoPrimitive)]
#[serde(into = "u32")]
#[repr(u32)]
pub enum Mode {
    Abort = 0,
    #[default]
    UnAuthenticated = 1,
    Authenticated = 2,
    Encrypted = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_valid_discriminants() {
        let reserved: u32 = Mode::Abort.into();
        let unauthenticated: u32 = Mode::UnAuthenticated.into();
        let authenticated: u32 = Mode::Authenticated.into();
        let encrypted: u32 = Mode::Encrypted.into();
        assert_eq!(reserved, 0u32);
        assert_eq!(unauthenticated, 1u32);
        assert_eq!(authenticated, 2u32);
        assert_eq!(encrypted, 4u32);
    }
}
