use std::fmt::Display;

use super::{error_estimate::ErrorEstimate, twamp_test_unauth::TwampTestPacketUnauth};
use crate::timestamp::timestamp::TimeStamp;
use deku::prelude::*;

/// The packet sent by Session-Reflector to Session-Sender.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct TwampTestPacketUnauthReflected {
    ///  The sequence number of the test packet according to its transmit order. It starts with
    ///  zero and is incremented by one for each subsequent packet.  The Sequence Number generated
    ///  by the Session-Reflector is independent from the sequence number of the arriving packets.
    pub sequence_number: u32,
    /// Timestamp when the reflected packet was sent from Session-Reflector.
    pub timestamp: TimeStamp,
    pub error_estimate: ErrorEstimate,
    #[deku(assert_eq = "0u16")]
    pub mbz_first: u16,
    /// Receive Timestamp is the time the test packet was received by the reflector. The difference
    /// between Timestamp and Receive Timestamp is the amount of time the packet was in transition
    /// in the Session-Reflector. The Error Estimate associated with the Timestamp field also
    /// applies to the Receive Timestamp.
    pub receive_timestamp: TimeStamp,
    /// Sender Sequence Number is a copy of the Sequence Number of the packet transmitted by the
    /// Session-Sender that caused the Session-Reflector to generate and send this test packet.
    pub sender_sequence_number: u32,
    /// Exact copy of `timestamp` from Session-Sender.
    pub sender_timestamp: TimeStamp,
    /// Exact copy of `ErrorEstimate` from Session-Sender.
    pub error_estimate_sender: ErrorEstimate,
    #[deku(assert_eq = "0u16")]
    pub mbz_second: u16,
    pub sender_ttl: u8,
    #[deku(count = "27")]
    pub packet_padding: Vec<u8>,
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
