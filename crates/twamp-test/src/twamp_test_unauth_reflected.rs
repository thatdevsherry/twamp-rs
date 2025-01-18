use std::fmt::Display;

use deku::prelude::*;
use timestamp::timestamp::TimeStamp;
use crate::error_estimate::ErrorEstimate;

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

impl TwampTestPacketUnauthReflected {}
