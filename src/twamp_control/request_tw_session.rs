use std::net::Ipv4Addr;

use super::command_number::CommandNumber;
use crate::timestamp::TimeStamp;
use deku::prelude::*;

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct RequestTwSession {
    /// Command number field. The value of Request-TW-Session is `5`.
    #[deku(assert_eq = "CommandNumber::RequestTwSession")]
    command_number: CommandNumber,

    /// Must be zero.
    #[deku(bits = "4", assert_eq = "0u8")]
    mbz_first: u8,

    /// IP version numbers for sender and receiver. Meaningful values are `4` and `6`.
    #[deku(bits = "4")]
    ipvn: u8,

    /// Used to inform Server to act as sender in TWAMP-Test.
    ///
    /// In TWAMP, it is always set to 0.
    conf_sender: u8,

    /// Used to inform Server to act as receiver in TWAMP-Test.
    ///
    /// In TWAMP, it is always set to 0.
    conf_receiver: u8,

    /// Used by Control-Client to determine when to send test packets.
    ///
    /// Must be zero in TWAMP as Session-Reflector does not process incoming packets and only
    /// reflects, and does not require this info.
    number_of_schedule_slots: u32,

    /// Number of active measurement packets to be sent during TWAMP-Session.
    ///
    /// Must be zero as Session-Reflector does not process incoming packets, therefore does not
    /// need to know the number of packets.
    number_of_packets: u32,

    /// UDP port on which Session-Sender will send from and receive TWAMP-Test packets.
    pub sender_port: u16,

    /// UDP port on which Session-Sender will send TWAMP-Test to and from where Session-Reflector
    /// will reflect packets.
    pub receiver_port: u16,

    /// IP address of sender. Can be set to 0 in which case the IP of Control-Client will be used.
    pub sender_address: Ipv4Addr,

    /// Utilised if [IPVN](Self::ipvn) is `6` otherwise is MBZ (Must Be Zero).
    sender_address_cont: [u8; 12],

    /// IP address of receiver. Can be set to 0 in which case the IP of Server will be used.
    pub receiver_address: Ipv4Addr,

    /// Utilised if [IPVN](Self::ipvn) is `6` otherwise is MBZ (Must Be Zero).
    receiver_address_cont: [u8; 12],

    /// Session Identifier. Must be 0 since it's generated on receiving side.
    sid: u128,

    /// Number of bytes to append to normal TWAMP-Test packet.
    pub padding_length: u32,

    /// Time when the session should be started. Cannot be before the time Start-Sessions is
    /// issued.
    pub start_time: TimeStamp,

    /// From [RFC 5357](https://datatracker.ietf.org/doc/html/rfc5357/#section-3.5):
    ///
    /// Timeout is the interval that the Session-Reflector MUST wait after receiving a
    /// Stop-Sessions message. In case there are test packets still in transit, the
    /// Session-Reflector MUST reflect them if they arrive within the Timeout interval following
    /// the reception of the Stop-Sessions message. The Session-Reflector MUST NOT reflect packets
    /// that are received beyond the timeout.
    pub timeout: u64,

    /// Set [DSCP](https://datatracker.ietf.org/doc/html/rfc2474).
    ///
    /// If present, the same value **must** be used in TWAMP-Test packets.
    type_p_descriptor: u32,

    octets_to_be_reflected: u16,
    length_of_padding_to_reflect: u16,

    /// MBZ (Must Be Zero).
    #[deku(assert_eq = "0u32")]
    mbz_last: u32,

    hmac: [u8; 16],
}

impl RequestTwSession {
    pub fn new(
        sender_address: Ipv4Addr,
        sender_port: u16,
        receiver_address: Ipv4Addr,
        receiver_port: u16,
        start_time: Option<TimeStamp>,
        timeout: u64,
    ) -> Self {
        RequestTwSession {
            command_number: CommandNumber::RequestTwSession,
            mbz_first: 0, // Must be zero.
            ipvn: 4,
            conf_sender: 0,              // Must be zero.
            conf_receiver: 0,            // Must be zero.
            number_of_schedule_slots: 0, // Must be zero.
            number_of_packets: 0,        // Must be zero.
            sender_port,
            receiver_port,
            sender_address,
            sender_address_cont: [0; 12],
            receiver_address,
            receiver_address_cont: [0; 12],
            sid: 0, // Must be zero.
            padding_length: 0,
            start_time: start_time.unwrap_or_default(),
            timeout,
            type_p_descriptor: 0,
            octets_to_be_reflected: 0,
            length_of_padding_to_reflect: 0,
            mbz_last: 0, // Must be zero.
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_TW_SESSION_LENGTH_IN_BYTES: usize = 112;

    #[test]
    fn command_number_is_correct() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(
            request_tw_session.command_number,
            CommandNumber::RequestTwSession
        );
    }

    #[test]
    fn first_mbz_are_zeros() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.mbz_first, 0u8);
    }

    /// Only support IPv4 for now.
    #[test]
    fn ipvn_is_correct() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.ipvn, 4u8);
    }

    #[test]
    fn conf_sender_is_zero() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.conf_sender, 0u8);
    }

    #[test]
    fn conf_receiver_is_zero() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.conf_receiver, 0u8);
    }

    #[test]
    fn number_of_schedule_slots_is_zero() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.number_of_schedule_slots, 0u32);
    }

    #[test]
    fn number_of_packets_is_zero() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.number_of_packets, 0u32);
    }

    #[test]
    fn sender_port_is_assigned() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            12345,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.sender_port, 12345);
    }

    #[test]
    fn receiver_port_is_assigned() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            12345,
            None,
            900,
        );
        assert_eq!(request_tw_session.receiver_port, 12345);
    }

    #[test]
    fn sender_address_is_assigned() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(
            request_tw_session.sender_address,
            Ipv4Addr::new(127, 0, 0, 1)
        );
    }

    /// Supporting only Ipv4 so this should be MBZ.
    #[test]
    fn sender_address_cont_is_mbz() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.sender_address_cont, [0; 12]);
    }

    #[test]
    fn receiver_address_is_assigned() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(
            request_tw_session.receiver_address,
            Ipv4Addr::new(127, 0, 0, 1)
        );
    }

    /// Supporting only Ipv4 so this should be MBZ.
    #[test]
    fn receiver_address_cont_is_mbz() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.receiver_address_cont, [0; 12]);
    }

    #[test]
    fn sid_is_zero() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.sid, 0);
    }

    #[test]
    #[ignore]
    fn padding_length_is_assigned() {
        todo!();
    }

    #[test]
    fn start_time_is_assigned() {
        let timestamp = TimeStamp::default();
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Some(timestamp),
            900,
        );
        assert_eq!(request_tw_session.start_time, timestamp);
    }

    #[test]
    #[ignore]
    fn timeout_is_assigned() {
        todo!();
    }

    #[test]
    #[ignore]
    fn type_p_descriptor_is_assigned() {
        todo!();
    }

    #[test]
    #[ignore]
    fn octets_to_be_reflected_is_assigned() {
        todo!();
    }

    #[test]
    #[ignore]
    fn length_of_padding_to_reflect_is_assigned() {
        todo!();
    }

    #[test]
    fn last_mbz_are_zeros() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        assert_eq!(request_tw_session.mbz_last, 0);
    }

    #[test]
    #[ignore]
    fn hmac_is_assigned() {
        todo!();
    }

    #[test]
    fn struct_serialized_has_size_acc_to_rfc() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        let encoded = request_tw_session.to_bytes().unwrap();
        assert_eq!(encoded.len(), REQUEST_TW_SESSION_LENGTH_IN_BYTES)
    }

    #[test]
    fn deserialize_to_struct() {
        let request_tw_session = RequestTwSession::new(
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            Ipv4Addr::new(127, 0, 0, 1),
            0,
            None,
            900,
        );
        let encoded = request_tw_session.to_bytes().unwrap();
        let (_rest, val) = RequestTwSession::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, request_tw_session)
    }
}
