use std::fmt;

use super::SecurityMode;
use deku::prelude::*;

/// Server Greeting sent by [`Server`](crate::server::Server) to [`Control-Client`](crate::control_client) after [`Control-Client`](crate::control_client) opens up a TCP
/// connection.
///
/// This is the first message in the TWAMP communication.
///
/// See details in [RFC 4656](https://datatracker.ietf.org/doc/html/rfc4656#section-3.1).
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct ServerGreeting {
    /// Same semantics as MBZ (Must Be Zero).
    #[deku(assert_eq = "[0u8; 12]")]
    unused: [u8; 12],

    /// [Security mode(s)](super::SecurityMode) that the [Server](crate::server::Server) supports.
    mode: u32,

    /// Unused in [unauthenticated mode](super::SecurityMode::Unauthenticated).
    ///
    /// It should be a random sequence of bytes.
    challenge: [u8; 16],

    /// Unused in [unauthenticated mode](super::SecurityMode::Unauthenticated).
    ///
    /// It is one of the parameters in deriving a key from a shared secret.
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
    pub const SERIALIZED_SIZE: usize = 64;

    /// Create greeting with `Modes` field set to bitwise OR of provided modes.
    ///
    /// # Example
    ///
    /// ```
    /// use twamp_rs::twamp_control::SecurityMode;
    /// use twamp_rs::twamp_control::ServerGreeting;
    ///
    /// let supported_modes = &[SecurityMode::Unauthenticated, SecurityMode::Authenticated];
    /// let server_greeting = ServerGreeting::new(supported_modes);
    /// ```
    pub fn new(modes: &[SecurityMode]) -> Self {
        ServerGreeting {
            unused: [0; 12],
            mode: modes.iter().fold(0u32, |acc, mode| {
                acc | <SecurityMode as Into<u32>>::into(*mode)
            }),
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
    /// use twamp_rs::twamp_control::SecurityMode;
    /// use twamp_rs::twamp_control::ServerGreeting;
    ///
    /// let my_count_value = 32769;
    /// let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]).with_count(my_count_value);
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
    /// use twamp_rs::twamp_control::SecurityMode;
    /// use twamp_rs::twamp_control::ServerGreeting;
    ///
    /// let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated, SecurityMode::Authenticated]);
    /// assert!(server_greeting.has_mode(SecurityMode::Unauthenticated));
    /// assert!(!server_greeting.has_mode(SecurityMode::Reserved));
    /// ```
    pub fn has_mode(&self, mode: SecurityMode) -> bool {
        let greeting_mode: u32 = self.mode;
        let mode_as_number: u32 = mode.into();
        match mode {
            SecurityMode::Reserved => greeting_mode | mode_as_number == mode_as_number,
            _ => greeting_mode & mode_as_number == mode_as_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn create_server_greeting_with_mode_reserved() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert_eq!(server_greeting.mode, 0);
    }

    #[test]
    fn read_mode_reserved_in_reserved_greeting() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(server_greeting.has_mode(SecurityMode::Reserved));
    }

    #[test]
    fn read_mode_reserved_in_non_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        assert!(!server_greeting.has_mode(SecurityMode::Reserved));
    }

    #[test]
    fn create_server_greeting_with_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        assert_eq!(server_greeting.mode, 1);
    }

    #[test]
    fn read_mode_unauthenticated() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        assert!(server_greeting.has_mode(SecurityMode::Unauthenticated));
    }

    #[test]
    fn create_server_greeting_with_mode_auth() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Authenticated]);
        assert_eq!(server_greeting.mode, 2);
    }

    #[test]
    fn read_mode_auth() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Authenticated]);
        assert!(server_greeting.has_mode(SecurityMode::Authenticated));
    }

    #[test]
    fn create_server_greeting_with_mode_encrypted() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Encrypted]);
        assert_eq!(server_greeting.mode, 4);
    }

    #[test]
    fn read_mode_encrypted() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Encrypted]);
        assert!(server_greeting.has_mode(SecurityMode::Encrypted));
    }

    #[test]
    fn create_server_greeting_with_mode_mixed() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::EncryptedControlUnauthTest]);
        assert_eq!(server_greeting.mode, 8);
    }

    #[test]
    fn read_mode_mixed() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::EncryptedControlUnauthTest]);
        assert!(server_greeting.has_mode(SecurityMode::EncryptedControlUnauthTest));
    }

    #[test]
    fn create_server_greeting_with_modes_unauth_and_auth_and_encrypted_and_mixed() {
        let server_greeting = ServerGreeting::new(&[
            SecurityMode::Unauthenticated,
            SecurityMode::Authenticated,
            SecurityMode::Encrypted,
            SecurityMode::EncryptedControlUnauthTest,
        ]);
        assert_eq!(server_greeting.mode, 15);
    }

    #[test]
    fn read_each_mode_from_unauth_and_auth_and_encrypted_and_mixed() {
        let server_greeting = ServerGreeting::new(&[
            SecurityMode::Unauthenticated,
            SecurityMode::Authenticated,
            SecurityMode::Encrypted,
            SecurityMode::EncryptedControlUnauthTest,
        ]);
        assert!(server_greeting.has_mode(SecurityMode::Unauthenticated));
        assert!(server_greeting.has_mode(SecurityMode::Authenticated));
        assert!(server_greeting.has_mode(SecurityMode::Encrypted));
        assert!(server_greeting.has_mode(SecurityMode::EncryptedControlUnauthTest));
    }

    #[test]
    fn read_mode_unauth_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(!server_greeting.has_mode(SecurityMode::Unauthenticated));
    }

    #[test]
    fn read_mode_auth_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(!server_greeting.has_mode(SecurityMode::Authenticated));
    }

    #[test]
    fn read_mode_encrypted_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(!server_greeting.has_mode(SecurityMode::Encrypted));
    }

    #[test]
    fn read_mode_mixed_in_reserved_greeting_and_fail() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(!server_greeting.has_mode(SecurityMode::EncryptedControlUnauthTest));
    }

    #[test]
    fn unused_are_zeros() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(server_greeting.unused == [0; 12]);
    }

    #[test]
    #[ignore] // we only support unauth mode as of now, leaving challenge as zeros.
    fn challenge_bytes_are_random() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        let challenge_bytes_unique = server_greeting.challenge.iter().collect::<HashSet<_>>();
        assert!(challenge_bytes_unique.len() > 1);
    }

    #[test]
    #[ignore] // we only support unauth mode as of now, leaving salt as zeros.
    fn salt_bytes_are_random() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        let challenge_bytes_unique = server_greeting.salt.iter().collect::<HashSet<_>>();
        assert!(challenge_bytes_unique.len() > 1);
    }

    #[test]
    fn default_count_is_under_a_valid_range() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(server_greeting.count.ge(&1024) && server_greeting.count.le(&32768));
    }

    #[test]
    fn use_custom_count_value() {
        let count_value = 32769;
        let server_greeting =
            ServerGreeting::new(&[SecurityMode::Reserved]).with_count(count_value);
        assert_eq!(server_greeting.count, count_value);
    }

    #[test]
    fn mbz_are_zeros() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Reserved]);
        assert!(server_greeting.mbz == [0; 12]);
    }

    #[test]
    fn serialize_into_correct_length_of_bytes() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        let encoded = server_greeting.to_bytes().unwrap();
        assert_eq!(encoded.len(), ServerGreeting::SERIALIZED_SIZE);
    }

    #[test]
    fn deserialize_into_correct_struct() {
        let server_greeting = ServerGreeting::new(&[SecurityMode::Unauthenticated]);
        let encoded = server_greeting.to_bytes().unwrap();
        let (_rest, val) = ServerGreeting::from_bytes((&encoded, 0)).unwrap();
        assert_eq!(val, server_greeting);
    }
}
