use deku::prelude::*;
use num_enum::IntoPrimitive;

/// Security Mode of TWAMP session. See details in
/// [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, Default, PartialEq, Copy, IntoPrimitive, DekuRead, DekuWrite)]
#[repr(u32)]
#[deku(id_type = "u32", endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub enum SecurityMode {
    /// Unused.
    ///
    /// [Control-Client](crate::control_client::ControlClient) **should** close the connection.
    ///
    /// [Server](crate::server::Server) **may** close the connection immediately.
    Reserved = 0,

    /// Unauthenticated [TWAMP-Control](super) and [TWAMP-Test](crate::twamp_test).
    #[default]
    Unauthenticated = 1,

    /// Authenticated [TWAMP-Control](super) and [TWAMP-Test](crate::twamp_test).
    Authenticated = 2,

    /// Encrypted [TWAMP-Control](super) and [TWAMP-Test](crate::twamp_test).
    Encrypted = 4,

    /// [Mixed security mode](https://datatracker.ietf.org/doc/html/rfc5618).
    /// Encrypted [TWAMP-Control](super) but unauthenticated [TWAMP-Test](crate::twamp_test).
    EncryptedControlUnauthTest = 8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_valid_discriminants() {
        let reserved: u32 = SecurityMode::Reserved.into();
        let unauthenticated: u32 = SecurityMode::Unauthenticated.into();
        let authenticated: u32 = SecurityMode::Authenticated.into();
        let encrypted: u32 = SecurityMode::Encrypted.into();
        assert_eq!(reserved, 0u32);
        assert_eq!(unauthenticated, 1u32);
        assert_eq!(authenticated, 2u32);
        assert_eq!(encrypted, 4u32);
    }
}
