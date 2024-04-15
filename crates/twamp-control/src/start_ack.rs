use crate::accept::Accept;
use deku::prelude::*;

/// Server Greeting sent by `Server` to `Control-Client` after `Control-Client` opens up a TCP
/// connection.
///
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct StartAck {
    accept: Accept,
    mbz: [u8; 15],
    hmac: [u8; 16],
}

impl StartAck {
    pub fn new(accept: Accept) -> Self {
        StartAck {
            accept,
            mbz: [0; 15],
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accept::Accept;
    const START_ACK_LENGTH_IN_BYTES: usize = 32;

    #[test]
    fn construct_with_accept_ok() {
        let accept = Accept::Ok;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, accept);
    }

    #[test]
    fn construct_with_accept_failure() {
        let accept = Accept::Failure;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, accept);
    }

    #[test]
    fn construct_with_accept_internal_error() {
        let accept = Accept::InternalError;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, accept);
    }

    #[test]
    fn construct_with_accept_not_supported() {
        let accept = Accept::NotSupported;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, Accept::NotSupported);
    }

    #[test]
    fn construct_with_accept_permanent_resource_limitation() {
        let accept = Accept::PermanentResourceLimitation;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, accept);
    }

    #[test]
    fn construct_with_accept_temporary_resource_limitation() {
        let accept = Accept::TemporaryResourceLimitation;
        let start_ack = StartAck::new(accept);
        assert_eq!(start_ack.accept, accept);
    }

    #[test]
    fn mbz_is_zero() {
        let start_ack = StartAck::new(Accept::Ok);
        assert_eq!(start_ack.mbz, [0; 15]);
    }

    #[test]
    #[ignore]
    fn hmac() {
        todo!()
    }

    #[test]
    fn serialize_to_bytes() {
        let start_ack = StartAck::new(Accept::Ok).to_bytes().unwrap();
        assert_eq!(start_ack.len(), START_ACK_LENGTH_IN_BYTES);
    }

    #[test]
    fn deserialize_to_struct() {
        let start_ack_as_bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let (_rest, start_ack) = StartAck::from_bytes((&start_ack_as_bytes, 0)).unwrap();
        assert_eq!(start_ack.accept, Accept::Ok);
        assert_eq!(start_ack.mbz, [0u8; 15]);
        assert_eq!(start_ack.hmac, [0u8; 16]);
    }
}
