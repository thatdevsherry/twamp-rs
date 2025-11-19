use super::Accept;
use crate::timestamp::TimeStamp;
use deku::prelude::*;
use std::time::Duration;

/// Sent by Server to Control-Client after receiving a [Set-Up-Response](crate::twamp_control::SetUpResponse) command.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ServerStart {
    /// MBZ (Must Be Zero).
    #[deku(assert_eq = "[0u8; 15]")]
    mbz_start: [u8; 15],

    /// Indicates Server's willingness to continue. See [list of possible values](Accept).
    accept: Accept,

    /// Generated randomly. Unused in
    /// [unauthenticated mode](crate::twamp_control::SecurityMode::Unauthenticated).
    server_iv: [u8; 16],

    /// The time when the Server binary was executed.
    start_time: TimeStamp,

    /// MBZ (Must Be Zero).
    #[deku(assert_eq = "[0u8; 8]")]
    mbz_end: [u8; 8],
}

impl ServerStart {
    pub const SERIALIZED_SIZE: usize = 48;

    /// Create instance with provided accept value.
    pub fn new(accept: Accept, start_time: Duration) -> Self {
        ServerStart {
            mbz_start: [0; 15],
            accept,
            server_iv: [0; 16], // unused in unauth mode
            start_time: TimeStamp::try_from(start_time)
                .expect("should have converted duration to timestamp."),
            mbz_end: [0; 8],
        }
    }

    /// Returns the value of Accept field.
    pub fn accept(&self) -> &Accept {
        &self.accept
    }

    /// Returns the value of Start-Time field.
    pub fn start_time(&self) -> &TimeStamp {
        &self.start_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const TIME: Duration = Duration::new(1713023152, 123456789);

    #[test]
    fn create_server_start_with_accept_ok() {
        let accept = Accept::Ok;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn create_server_start_with_accept_failure() {
        let accept = Accept::Failure;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn create_server_start_with_accept_internal_error() {
        let accept = Accept::InternalError;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn create_server_start_with_accept_not_supported() {
        let accept = Accept::NotSupported;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn create_server_start_with_accept_permanent_resource_limitation() {
        let accept = Accept::PermanentResourceLimitation;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn create_server_start_with_accept_temporary_resource_limitation() {
        let accept = Accept::TemporaryResourceLimitation;
        let server_start = ServerStart::new(accept, TIME);
        let server_start_accept: u8 = server_start.accept.into();
        assert_eq!(server_start_accept, accept.into());
    }

    #[test]
    fn first_mbz_are_zeros() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        assert!(server_start.mbz_start == [0; 15]);
    }

    #[test]
    fn last_mbz_are_zeros() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        assert!(server_start.mbz_end == [0; 8]);
    }

    #[test]
    #[ignore]
    fn server_iv_is_random() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let server_iv_bytes_unique = server_start.server_iv.iter().collect::<HashSet<_>>();
        assert!(server_iv_bytes_unique.len() > 1);
    }

    #[test]
    fn should_serialize_to_correct_bytes() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let encoded = server_start.to_bytes().unwrap();
        assert_eq!(encoded.len(), ServerStart::SERIALIZED_SIZE);
    }

    #[test]
    fn should_deserialize_to_correct_struct() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let encoded = server_start.to_bytes().unwrap();
        let (_rest, val) = ServerStart::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, server_start);
    }
}
