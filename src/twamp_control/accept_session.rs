use super::Accept;
use deku::prelude::*;

/// Response for a Request-TW-Session command.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AcceptSession {
    /// Represents Server's willingness to continue or reject.
    pub accept: Accept,

    /// MBZ (Must Be Zero).
    #[deku(assert_eq = "0u8")]
    mbz_first: u8,

    /// Either the port that was present in Request-TW-Session or an alternative port in case the
    /// requested port by Control-Client is not available.
    pub port: u16,

    pub sid: [u8; 16],

    /// Should reconfirm the number of octets to reflect, which was provided in Request-TW-Session.
    pub reflected_octets: u16,

    /// Set by server to inform Control-Client about what octets the Server expects in TWAMP-Test
    /// packets padding.
    ///
    /// If Server doesn't need octets to be returned in TWAMP-Test packets, this should be zero.
    ///
    /// If Server intends octets to be reflected, this field should be non-zero. The value set here
    /// will be sent in the TWAMP-Test packets.
    pub server_octets: u16,

    /// MBZ (Must Be Zero).
    #[deku(assert_eq = "[0u8; 8]")]
    mbz_second: [u8; 8],

    pub hmac: [u8; 16],
}

impl AcceptSession {
    /// Construct from an Accept value and port. It sets sid and hmac as zeros.
    pub fn new(accept: Accept, port: u16, reflected_octets: u16, server_octets: u16) -> Self {
        AcceptSession {
            accept,
            mbz_first: 0,
            port,
            sid: [0; 16], // TODO: impl. when using pnet/pcap or something.
            reflected_octets,
            server_octets,
            mbz_second: [0; 8],
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ACCEPT_SESSION_LENGTH_IN_BYTES: usize = 48;

    #[test]
    fn construct_with_accept_ok() {
        let accept = Accept::Ok;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn construct_with_accept_failure() {
        let accept = Accept::Failure;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn construct_with_accept_internal_error() {
        let accept = Accept::InternalError;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn construct_with_accept_not_supported() {
        let accept = Accept::NotSupported;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn construct_with_accept_permanent_resource_limitation() {
        let accept = Accept::PermanentResourceLimitation;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn construct_with_accept_temporary_resource_limitation() {
        let accept = Accept::TemporaryResourceLimitation;
        let accept_session = AcceptSession::new(accept, 0, 0, 0);
        assert_eq!(accept_session.accept, accept);
    }

    #[test]
    fn port_is_assigned() {
        let port = 12345u16;
        let accept_session = AcceptSession::new(Accept::Ok, port, 0, 0);
        assert_eq!(accept_session.port, port);
    }

    #[test]
    #[ignore]
    fn sid_is_random() {
        todo!();
    }

    #[test]
    fn reflected_octets_is_assigned() {
        let reflected_octets = 0;
        let accept_session = AcceptSession::new(Accept::Ok, 0, reflected_octets, 0);
        assert_eq!(accept_session.reflected_octets, reflected_octets);
    }

    #[test]
    fn server_octets_is_assigned() {
        let server_octets = 0;
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, server_octets);
        assert_eq!(accept_session.server_octets, server_octets);
    }

    #[test]
    fn first_mbz_is_zero() {
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, 0);
        assert_eq!(accept_session.mbz_first, 0);
    }

    #[test]
    fn second_mbz_is_zero() {
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, 0);
        assert_eq!(accept_session.mbz_second, [0; 8]);
    }

    #[test]
    fn should_serialize_into_correct_length_of_bytes() {
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, 0);
        let encoded = accept_session.to_bytes().unwrap();
        assert_eq!(encoded.len(), ACCEPT_SESSION_LENGTH_IN_BYTES);
    }

    #[test]
    fn should_deserialize_into_correct_length_of_bytes() {
        let accept_session = AcceptSession::new(Accept::Ok, 0, 0, 0);
        let encoded = accept_session.to_bytes().unwrap();
        let (_rest, val) = AcceptSession::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, accept_session);
    }
}
