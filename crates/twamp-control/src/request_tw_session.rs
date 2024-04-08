use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use crate::{command_number::CommandNumber, timestamp::TimeStamp};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RequestTwSession {
    /// Command number field. The value of Request-TW-Session is `5`.
    pub command_number: u8,

    /// IP version numbers for sender and receiver. Meaningful values are `4` and `6`.
    ///
    /// This field also combines the MBZ before IPVN. Since MBZ & IPVN are 4-bits each, we can
    /// represent them both using `u8`, which will add padding of 5 zeros and use the last 3 digits
    /// for representing IPVN.
    pub ipvn: u8,

    /// Used to inform Server to act as sender in TWAMP-Test.
    ///
    /// In TWAMP, it is always set to 0.
    pub conf_sender: u8,

    /// Used to inform Server to act as receiver in TWAMP-Test.
    ///
    /// In TWAMP, it is always set to 0.
    pub conf_receiver: u8,

    /// Used by Control-Client to determine when to send test packets.
    ///
    /// Must be zero in TWAMP as Session-Reflector does not process incoming packets and only
    /// reflects, and does not require this info.
    pub number_of_schedule_slots: u32,

    /// Number of active measurement packets to be sent during TWAMP-Session.
    ///
    /// Must be zero as Session-Reflector does not process incoming packets, therefore does not
    /// need to know the number of packets.
    pub number_of_packets: u32,

    /// UDP port on which Session-Sender will send from and receive TWAMP-Test packets.
    pub sender_port: u16,

    /// UDP port on which Session-Sender will send TWAMP-Test to and from where Session-Reflector
    /// will reflect packets.
    pub receiver_port: u16,

    /// IP address of sender. Can be set to 0 in which case the IP of Control-Client will be used.
    pub sender_address: u32,

    /// Utilised if [IPVN](Self::ipvn) is `6` otherwise is MBZ (Must Be Zero).
    pub sender_address_cont: [u8; 12],

    /// IP address of receiver. Can be set to 0 in which case the IP of Server will be used.
    pub receiver_address: u32,

    /// Utilised if [IPVN](Self::ipvn) is `6` otherwise is MBZ (Must Be Zero).
    pub receiver_address_cont: [u8; 12],

    /// Session Identifier. Must be 0 since it's generated on receiving side.
    pub sid: [u8; 16],

    /// Number of bytes to append to normal TWAMP-Test packet.
    pub padding_length: u32,

    /// Time when the session should be started. Cannot be before the time Start-Sessions is
    /// issued.
    pub start_time: TimeStamp,

    /// From [RFC 5357](https://datatracker.ietf.org/doc/html/rfc5357/#section-3.5):
    ///
    /// > Timeout is the interval that the Session-Reflector MUST wait after receiving a
    /// Stop-Sessions message. In case there are test packets still in transit, the
    /// Session-Reflector MUST reflect them if they arrive within the Timeout interval following
    /// the reception of the Stop-Sessions message. The Session-Reflector MUST NOT reflect packets
    /// that are received beyond the timeout.
    pub timeout: [u8; 8],

    /// Set [DSCP](https://datatracker.ietf.org/doc/html/rfc2474).
    ///
    /// If present, the same value **must** be used in TWAMP-Test packets.
    pub type_p_descriptor: u32,

    /// MBZ (Must Be Zero).
    pub mbz: [u8; 8],

    pub hmac: [u8; 16],
}

impl From<&TcpStream> for RequestTwSession {
    fn from(value: &TcpStream) -> Self {
        let request_tw_session = RequestTwSession::new(CommandNumber::RequestTwSession);
        let sender_address: u32 = match value.local_addr().unwrap().ip() {
            IpAddr::V4(ip) => ip,
            IpAddr::V6(ip) => panic!("da hail did v6 come from: {ip}"),
        }
        .into();
        let sender_port = value.local_addr().unwrap().port();
        let receiver_address: u32 = match value.peer_addr().unwrap().ip() {
            IpAddr::V4(ip) => ip,
            IpAddr::V6(ip) => panic!("da hail did v6 come from: {ip}"),
        }
        .into();
        let receiver_port = value.peer_addr().unwrap().port();
        RequestTwSession {
            sender_address,
            sender_port,
            receiver_address,
            receiver_port,
            ..request_tw_session
        }
    }
}

impl RequestTwSession {
    pub fn new(command_number: CommandNumber) -> Self {
        RequestTwSession {
            command_number: command_number as u8,
            ipvn: 4,

            // Must be zero.
            conf_sender: 0,
            // Must be zero.
            conf_receiver: 0,
            // Must be zero.
            number_of_schedule_slots: 0,
            // Must be zero.
            number_of_packets: 0,

            sender_port: 0,
            receiver_port: 0,
            sender_address: 0,
            sender_address_cont: [0; 12],
            receiver_address: 0,
            receiver_address_cont: [0; 12],
            // Must be zero.
            sid: [0; 16],
            padding_length: 0,
            start_time: TimeStamp::new(),
            timeout: [0; 8],
            type_p_descriptor: 0,
            mbz: [0; 8],
            hmac: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use bincode::Options;

    use crate::request_tw_session::RequestTwSession;

    #[test]
    fn should_have_correct_size() {
        assert_eq!(size_of::<RequestTwSession>(), 112)
    }

    #[test]
    fn should_serialize_correctly() {
        let request_tw_session = RequestTwSession::default();
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&request_tw_session)
            .unwrap();
        assert_eq!(encoded.len(), 112)
    }

    #[test]
    fn should_deserialize_to_struct() {
        let request_tw_session = RequestTwSession::default();
        let encoded = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .serialize(&request_tw_session)
            .unwrap();
        println!("{}", encoded.len());
        let decoded: RequestTwSession = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .deserialize(&encoded)
            .unwrap();
        assert_eq!(decoded, request_tw_session)
    }
}
