use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use crate::server_start::TimeStamp;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RequestTwSession {
    command_number: u8,

    /// This field also combines the MBZ before IPVN.
    /// Since MBZ & IPVN are 4-bits each, we can represent
    /// them both using u8, which will add 5 zero padding
    /// and use the last 3 digits for representing IPVN.
    ipvn: u8,

    conf_sender: u8,

    conf_receiver: u8,

    number_of_schedule_slots: u32,

    number_of_packets: u32,
    sender_port: u16,
    receiver_port: u16,
    sender_address: u32,
    sender_address_cont: u128,
    receiver_address: u32,
    receiver_address_cont: u128,
    sid: [u8; 16],
    padding_length: u32,
    start_time: TimeStamp,
    timeout: [u8; 8],
    type_p_descriptor: u32,
    mbz: [u8; 8],
    hmac: [u8; 16],
}

impl From<&TcpStream> for RequestTwSession {
    fn from(value: &TcpStream) -> Self {
        let request_tw_session = RequestTwSession::new();
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
    pub fn new() -> Self {
        RequestTwSession {
            command_number: 5,
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
            sender_address_cont: 0,
            receiver_address: 0,
            receiver_address_cont: 0,
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
