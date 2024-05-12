#![allow(dead_code)]

use timestamp::timestamp::TimeStamp;

/// The packet sent by Session-Sender to Session-Reflector.
pub struct TwampTestPacketUnauth {
    sequence_number: u32,
    timestamp: TimeStamp,
    error_estimate: u16,
    packet_padding: u16,
}

impl TwampTestPacketUnauth {}

/// The packet sent by Session-Reflector to Session-Sender.
pub struct TwampTestPacketUnauthReflected {
    sequence_number: u32,
    timestamp: TimeStamp,
    error_estimate: u16,
    mbz_first: u16,
    receive_timestamp: TimeStamp,
    sender_sequence_number: u32,
    sender_timestamp: TimeStamp,
    error_estimate_sender: u16,
    mbz_second: u16,
    sender_ttl: u8,
    packet_padding: u32,
}

impl TwampTestPacketUnauthReflected {}

#[cfg(test)]
mod tests {
    use super::*;
}
