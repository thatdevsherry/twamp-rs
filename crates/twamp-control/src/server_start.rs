use std::time::Duration;

use crate::{accept::Accept, timestamp::TimeStamp};
use deku::prelude::*;
use rand::random;

/// Sent by Server to Control-Client after receiving a [Set-Up-Response](crate::set_up_response::SetUpResponse) command.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ServerStart {
    /// MBZ (Must Be Zero).
    mbz_start: [u8; 15],

    /// Indicates Server's willingness to continue. See [list of possible values](Accept).
    accept: Accept,

    /// Generated randomly. Unused in
    /// [unauthenticated mode](crate::security_mode::Mode::Unauthenticated).
    server_iv: [u8; 16],

    /// The time when the Server binary was executed.
    start_time: TimeStamp,

    /// MBZ (Must Be Zero).
    mbz_end: [u8; 8],
}

impl ServerStart {
    /// Create instance with provided accept value.
    pub fn new(accept: Accept, start_time: Duration) -> Self {
        ServerStart {
            mbz_start: [0; 15],
            accept,
            server_iv: Vec::from([0; 16])
                .iter()
                .map(|_| random())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
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
    use std::{collections::HashSet, mem::size_of};

    const SERVER_START_LENGTH_IN_BYTES: usize = 48;
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
    fn server_iv_is_random() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let server_iv_bytes_unique = server_start.server_iv.iter().collect::<HashSet<_>>();
        assert!(server_iv_bytes_unique.len() > 1);
    }

    #[test]
    fn should_have_correct_bytes_of_struct() {
        assert_eq!(size_of::<ServerStart>(), SERVER_START_LENGTH_IN_BYTES);
    }

    #[test]
    fn should_serialize_to_correct_bytes() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let encoded = server_start.to_bytes().unwrap();
        assert_eq!(encoded.len(), SERVER_START_LENGTH_IN_BYTES);
    }

    #[test]
    fn should_deserialize_to_correct_struct() {
        let server_start = ServerStart::new(Accept::Ok, TIME);
        let encoded = server_start.to_bytes().unwrap();
        let (_rest, val) = ServerStart::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, server_start);
    }
}
