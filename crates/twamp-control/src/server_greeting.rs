use std::fmt;

use crate::security_mode::Mode;
use deku::prelude::*;
use rand::random;

/// Server Greeting sent by `Server` to `Control-Client` after `Control-Client` opens up a TCP
/// connection.
///
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ServerGreeting {
    /// Same semantics as MBZ (Must Be Zero).
    #[deku(assert_eq = "[0u8; 12]")]
    unused: [u8; 12],

    /// Security mode(s) that the Server supports.
    mode: u32,

    /// Random seq of bytes.
    challenge: [u8; 16],

    /// Random seq of bytes.
    salt: [u8; 16],

    /// TWAMP sets default MAX value SHOULD be 32768. It can be increased if computing
    /// power can handle.
    count: u32,

    /// Must Be Zero.
    #[deku(assert_eq = "[0u8; 12]")]
    mbz: [u8; 12],
}

impl fmt::Display for ServerGreeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Greeting with Mode and Count: {}, {}",
            self.mode, self.count
        )
    }
}

impl ServerGreeting {
    /// Create greeting with `Modes` field set to bitwise OR of provided modes.
    ///
    /// # Example
    ///
    /// ```
    /// use twamp_control::security_mode::Mode;
    /// use twamp_control::server_greeting::ServerGreeting;
    ///
    /// let supported_modes = &[Mode::Unauthenticated, Mode::Authenticated];
    /// let server_greeting = ServerGreeting::new(supported_modes);
    /// ```
    pub fn new(modes: &[Mode]) -> Self {
        ServerGreeting {
            unused: [0; 12],
            mode: modes
                .iter()
                .fold(0u32, |acc, mode| acc | <Mode as Into<u32>>::into(*mode)),
            challenge: Vec::from([0; 16])
                .iter()
                .map(|_| random())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
            salt: Vec::from([0; 16])
                .iter()
                .map(|_| random())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
            count: 1024,
            mbz: [0; 12],
        }
    }

    /// Use the provided count value in the greeting.
    ///
    /// # Example usage
    ///
    /// ```
    /// use twamp_control::security_mode::Mode;
    /// use twamp_control::server_greeting::ServerGreeting;
    ///
    /// let my_count_value = 32769;
    /// let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]).with_count(my_count_value);
    /// assert_eq!(server_greeting.count(), my_count_value);
    /// ```
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    /// Get the value of count field.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Checks if the provided mode exists in greeting's `Mode` field.
    ///
    /// ```
    /// use twamp_control::security_mode::Mode;
    /// use twamp_control::server_greeting::ServerGreeting;
    ///
    /// let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated, Mode::Authenticated]);
    /// assert!(server_greeting.has_mode(Mode::Unauthenticated));
    /// assert!(!server_greeting.has_mode(Mode::Reserved));
    /// ```
    pub fn has_mode(&self, mode: Mode) -> bool {
        let greeting_mode: u32 = self.mode;
        let mode_as_number: u32 = mode.into();
        match mode {
            Mode::Reserved => greeting_mode | mode_as_number == mode_as_number,
            _ => greeting_mode & mode_as_number == mode_as_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const SERVER_GREETING_LENGTH_IN_BYTES: usize = 64;

    #[test]
    fn create_server_greeting_with_mode_reserved() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert_eq!(server_greeting.mode, 0);
    }

    #[test]
    fn read_mode_reserved_in_reserved_greeting() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(server_greeting.has_mode(Mode::Reserved));
    }

    #[test]
    fn read_mode_reserved_in_non_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        assert!(!server_greeting.has_mode(Mode::Reserved));
    }

    #[test]
    fn create_server_greeting_with_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        assert_eq!(server_greeting.mode, 1);
    }

    #[test]
    fn read_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        assert!(server_greeting.has_mode(Mode::Unauthenticated));
    }

    #[test]
    fn create_server_greeting_with_mode_auth() {
        let server_greeting = ServerGreeting::new(&[Mode::Authenticated]);
        assert_eq!(server_greeting.mode, 2);
    }

    #[test]
    fn read_mode_auth() {
        let server_greeting = ServerGreeting::new(&[Mode::Authenticated]);
        assert!(server_greeting.has_mode(Mode::Authenticated));
    }

    #[test]
    fn create_server_greeting_with_mode_encrypted() {
        let server_greeting = ServerGreeting::new(&[Mode::Encrypted]);
        assert_eq!(server_greeting.mode, 4);
    }

    #[test]
    fn read_mode_encrypted() {
        let server_greeting = ServerGreeting::new(&[Mode::Encrypted]);
        assert!(server_greeting.has_mode(Mode::Encrypted));
    }

    #[test]
    fn create_server_greeting_with_mode_mixed() {
        let server_greeting = ServerGreeting::new(&[Mode::EncryptedControlUnauthTest]);
        assert_eq!(server_greeting.mode, 8);
    }

    #[test]
    fn read_mode_mixed() {
        let server_greeting = ServerGreeting::new(&[Mode::EncryptedControlUnauthTest]);
        assert!(server_greeting.has_mode(Mode::EncryptedControlUnauthTest));
    }

    #[test]
    fn create_server_greeting_with_modes_unauth_and_auth_and_encrypted_and_mixed() {
        let server_greeting = ServerGreeting::new(&[
            Mode::Unauthenticated,
            Mode::Authenticated,
            Mode::Encrypted,
            Mode::EncryptedControlUnauthTest,
        ]);
        assert_eq!(server_greeting.mode, 15);
    }

    #[test]
    fn read_each_mode_from_unauth_and_auth_and_encrypted_and_mixed() {
        let server_greeting = ServerGreeting::new(&[
            Mode::Unauthenticated,
            Mode::Authenticated,
            Mode::Encrypted,
            Mode::EncryptedControlUnauthTest,
        ]);
        assert!(server_greeting.has_mode(Mode::Unauthenticated));
        assert!(server_greeting.has_mode(Mode::Authenticated));
        assert!(server_greeting.has_mode(Mode::Encrypted));
        assert!(server_greeting.has_mode(Mode::EncryptedControlUnauthTest));
    }

    #[test]
    fn read_mode_unauth_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(!server_greeting.has_mode(Mode::Unauthenticated));
    }

    #[test]
    fn read_mode_auth_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(!server_greeting.has_mode(Mode::Authenticated));
    }

    #[test]
    fn read_mode_encrypted_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(!server_greeting.has_mode(Mode::Encrypted));
    }

    #[test]
    fn read_mode_mixed_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(!server_greeting.has_mode(Mode::EncryptedControlUnauthTest));
    }

    #[test]
    fn unused_are_zeros() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(server_greeting.unused == [0; 12]);
    }

    #[test]
    fn challenge_bytes_are_random() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        let challenge_bytes_unique = server_greeting.challenge.iter().collect::<HashSet<_>>();
        assert!(challenge_bytes_unique.len() > 1);
    }

    #[test]
    fn salt_bytes_are_random() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        let challenge_bytes_unique = server_greeting.salt.iter().collect::<HashSet<_>>();
        assert!(challenge_bytes_unique.len() > 1);
    }

    #[test]
    fn default_count_is_under_a_valid_range() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(server_greeting.count.ge(&1024) && server_greeting.count.le(&32768));
    }

    #[test]
    fn use_custom_count_value() {
        let count_value = 32769;
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]).with_count(count_value);
        assert_eq!(server_greeting.count, count_value);
    }

    #[test]
    fn mbz_are_zeros() {
        let server_greeting = ServerGreeting::new(&[Mode::Reserved]);
        assert!(server_greeting.mbz == [0; 12]);
    }

    #[test]
    fn serialize_into_correct_length_of_bytes() {
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        let encoded = server_greeting.to_bytes().unwrap();
        assert_eq!(encoded.len(), SERVER_GREETING_LENGTH_IN_BYTES);
    }

    #[test]
    fn deserialize_into_correct_struct() {
        let server_greeting = ServerGreeting::new(&[Mode::Unauthenticated]);
        let encoded = server_greeting.to_bytes().unwrap();
        let (_rest, val) = ServerGreeting::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, server_greeting);
    }
}
