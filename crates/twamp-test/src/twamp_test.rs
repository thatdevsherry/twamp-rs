#![allow(dead_code)]

pub struct TwampTestUnAuthenticated {
    sequence_number: [u8; 32],
    timestamp: [u8; 64],
    error_estimate: [u8; 16],
    mbz_first: [u8; 16],
    receive_timestamp: [u8; 64],
    sender_sequence_number: [u8; 32],
    sender_timestamp: [u8; 64],
    error_estimate_sender: [u8; 16],
    mbz_second: [u8; 16],
    sender_ttl: u8,
    packet_padding: [u8; 536],
}
