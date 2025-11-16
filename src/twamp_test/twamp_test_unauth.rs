use std::fmt::Display;

use super::error_estimate::ErrorEstimate;
use crate::timestamp::TimeStamp;
use deku::prelude::*;

/// The packet sent by Session-Sender to Session-Reflector.
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct TwampTestPacketUnauth {
    pub sequence_number: u32,
    pub timestamp: TimeStamp,
    pub error_estimate: ErrorEstimate,
    #[deku(count = "27", assert = "packet_padding.len() <= 27")]
    pub packet_padding: Vec<u8>,
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

    /// Creates a new Twamp-Test packet to be sent by Session-Sender.
    ///
    /// Note that the padding length is from `0-27`.
    /// It will resort to `27` even if a value greater
    /// than `27` is passed.
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
}
