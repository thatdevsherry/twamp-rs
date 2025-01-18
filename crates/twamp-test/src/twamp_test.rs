#![allow(dead_code)]

use std::fmt::Display;

use deku::prelude::*;
use timestamp::timestamp::TimeStamp;

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ErrorEstimate {
    /// SHOULD be set if the party generating the timestamp has a clock that is synchronized to UTC
    /// using an external source (e.g., the bit should be set if GPS hardware is used and it
    /// indicates that it has acquired current position and time or if NTP is used and it indicates
    /// that it has synchronized to an external source, which includes stratum 0 source, etc.).
    /// If there is no notion of external synchronization for the time source, the bit SHOULD NOT
    /// be set.
    #[deku(bits = "1")]
    s: u8,

    /// Same semantics as MBZ fields elsewhere: it MUST be set to zero by the sender and ignored
    /// by everyone else.
    #[deku(bits = "1", assert_eq = "0u8")]
    mbz: u8,

    /// An unsigned integer.
    #[deku(bits = "6")]
    scale: u8,

    /// Multiplier is an unsigned integer as well. They are interpreted
    /// as follows: the error estimate is equal to
    /// Multiplier*2^(-32)*2^Scale (in seconds).
    /// (Notation clarification: 2^Scale is two to the power of Scale.)
    /// Multiplier MUST NOT be set to zero.
    /// If Multiplier is zero, the packet SHOULD be considered corrupt and discarded.
    multiplier: u8,
}

impl ErrorEstimate {
    fn new(ntp_synchronized: bool) -> ErrorEstimate {
        ErrorEstimate {
            s: if ntp_synchronized { 1 } else { 0 },
            mbz: 0,
            scale: if ntp_synchronized { 0 } else { 63 },
            multiplier: if ntp_synchronized { 1 } else { 255 },
        }
    }
}

/// The packet sent by Session-Sender to Session-Reflector.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct TwampTestPacketUnauth {
    sequence_number: u32,
    timestamp: TimeStamp,
    error_estimate: ErrorEstimate,
    #[deku(count = "27", assert = "packet_padding.len() <= 27")]
    packet_padding: Vec<u8>,
}

impl Display for TwampTestPacketUnauth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Twamp-Test sender packet with sequence: {}",
            self.sequence_number
        )
    }
}

impl TwampTestPacketUnauth {
    const MAX_PADDING_LENGTH: u8 = 27;

    pub fn new(sequence_number: u32, padding_length: u8, is_ntp_synchronized: bool) -> Self {
        TwampTestPacketUnauth {
            sequence_number,
            timestamp: TimeStamp::default(),
            error_estimate: ErrorEstimate::new(is_ntp_synchronized),
            // NOTE: Using 27 as the max value even if > 27 was passed in padding.
            packet_padding: vec![
                0;
                if padding_length > 27 {
                    Self::MAX_PADDING_LENGTH.into()
                } else {
                    padding_length.into()
                }
            ],
        }
    }
}

/// The packet sent by Session-Reflector to Session-Sender.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct TwampTestPacketUnauthReflected {
    sequence_number: u32,
    timestamp: TimeStamp,
    error_estimate: u16,
    #[deku(assert_eq = "0u16")]
    mbz_first: u16,
    receive_timestamp: TimeStamp,
    sender_sequence_number: u32,
    sender_timestamp: TimeStamp,
    error_estimate_sender: u16,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_twamp_test_packet_with_sequence_number() {
        let test_packet_sender = TwampTestPacketUnauth::new(1, 27, true);
        assert_eq!(test_packet_sender.sequence_number, 1);
    }

    #[test]
    fn create_twamp_test_packet_with_min_padding() {
        let padding_length = 0;
        let test_packet_sender = TwampTestPacketUnauth::new(1, padding_length, true);
        assert_eq!(
            test_packet_sender.packet_padding.len(),
            padding_length.into()
        );
    }

    #[test]
    fn create_twamp_test_packet_with_max_padding() {
        let padding_length = 27;
        let test_packet_sender = TwampTestPacketUnauth::new(1, padding_length, true);
        assert_eq!(
            test_packet_sender.packet_padding.len(),
            padding_length.into()
        );
    }

    #[test]
    fn create_twamp_test_packet_with_overflow_padding() {
        let padding_length = 255;
        let test_packet_sender = TwampTestPacketUnauth::new(1, padding_length, true);
        assert_eq!(
            test_packet_sender.packet_padding.len(),
            TwampTestPacketUnauth::MAX_PADDING_LENGTH.into()
        );
    }

    #[test]
    fn create_twamp_test_packet_with_ntp_synchronized() {
        let padding_length = 0;
        let test_packet_sender = TwampTestPacketUnauth::new(1, padding_length, true);
        assert_eq!(test_packet_sender.error_estimate.s, 1);
        assert_eq!(test_packet_sender.error_estimate.mbz, 0);
        assert_eq!(test_packet_sender.error_estimate.scale, 0);
        assert_eq!(test_packet_sender.error_estimate.multiplier, 1);
    }

    #[test]
    fn create_twamp_test_packet_with_ntp_not_synchronized() {
        let padding_length = 255;
        let test_packet_sender = TwampTestPacketUnauth::new(1, padding_length, false);
        assert_eq!(test_packet_sender.error_estimate.s, 0);
        assert_eq!(test_packet_sender.error_estimate.mbz, 0);
        assert_eq!(test_packet_sender.error_estimate.scale, 63);
        assert_eq!(test_packet_sender.error_estimate.multiplier, 255);
    }
}
