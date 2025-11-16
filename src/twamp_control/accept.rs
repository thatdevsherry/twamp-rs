use deku::prelude::*;
use num_enum::IntoPrimitive;

/// Used to communicate Server responses to Control-Client throughout TWAMP-Control protocol.
#[derive(Clone, Debug, Default, PartialEq, Copy, IntoPrimitive, DekuRead, DekuWrite)]
#[repr(u8)]
#[deku(id_type = "u8", endian = "endian", ctx = "endian: deku::ctx::Endian")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_valid_discriminants() {
        let ok: u8 = Accept::Ok.into();
        let failure: u8 = Accept::Failure.into();
        let internal_error: u8 = Accept::InternalError.into();
        let not_supported: u8 = Accept::NotSupported.into();
        let permanent_resource_limitation: u8 = Accept::PermanentResourceLimitation.into();
        let temporary_resource_limitation: u8 = Accept::TemporaryResourceLimitation.into();
        assert_eq!(ok, 0u8);
        assert_eq!(failure, 1u8);
        assert_eq!(internal_error, 2u8);
        assert_eq!(not_supported, 3u8);
        assert_eq!(permanent_resource_limitation, 4u8);
        assert_eq!(temporary_resource_limitation, 5u8);
    }
}
