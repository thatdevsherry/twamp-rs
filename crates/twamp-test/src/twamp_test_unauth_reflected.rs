use std::fmt::Display;

use crate::{error_estimate::ErrorEstimate, twamp_test_unauth::TwampTestPacketUnauth};
use deku::prelude::*;
use timestamp::timestamp::TimeStamp;

/// The packet sent by Session-Reflector to Session-Sender.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct TwampTestPacketUnauthReflected {
    sequence_number: u32,
    timestamp: TimeStamp,
    error_estimate: ErrorEstimate,
    #[deku(assert_eq = "0u16")]
    mbz_first: u16,
    receive_timestamp: TimeStamp,
    sender_sequence_number: u32,
    sender_timestamp: TimeStamp,
    error_estimate_sender: ErrorEstimate,
    #[deku(assert_eq = "0u16")]
    mbz_second: u16,
    sender_ttl: u8,
    #[deku(count = "27")]
    packet_padding: Vec<u8>,
}

impl Display for TwampTestPacketUnauthReflected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Twamp-Test reflected packet with sequence: {}",
            self.sequence_number
        )
    }
}

impl TwampTestPacketUnauthReflected {
    pub fn new(seq: u32, twamp_test_pkt: TwampTestPacketUnauth, recv_ts: TimeStamp) -> Self {
        TwampTestPacketUnauthReflected {
            sequence_number: seq,
            timestamp: TimeStamp::default(),
            error_estimate: ErrorEstimate::new(true),
            mbz_first: 0,
            receive_timestamp: recv_ts,
            sender_sequence_number: twamp_test_pkt.sequence_number,
            sender_timestamp: twamp_test_pkt.timestamp,
            error_estimate_sender: twamp_test_pkt.error_estimate,
            mbz_second: 0,
            sender_ttl: 255, // TODO: hard-coded
            packet_padding: vec![0; 0],
        }
    }
}
