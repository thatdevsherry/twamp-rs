use num_enum::IntoPrimitive;
use serde::{Deserialize, Serialize};

/// Values of Command Number.
///
/// Defined in [RFC 5357](https://datatracker.ietf.org/doc/html/rfc5357/#section-8.4).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, IntoPrimitive)]
#[repr(u8)]
#[serde(into = "u8")]
pub enum CommandNumber {
    Forbidden = 1,
    StartSession = 2,
    StopSession = 3,
    RequestTwSession = 5,
    Experimentation = 6,
}

#[cfg(test)]
mod tests {
    use super::CommandNumber;

    #[test]
    fn should_have_valid_discriminants() {
        let forbidden: u8 = CommandNumber::Forbidden.into();
        let start_session: u8 = CommandNumber::StartSession.into();
        let stop_session: u8 = CommandNumber::StopSession.into();
        let request_tw_session: u8 = CommandNumber::RequestTwSession.into();
        let experimentation: u8 = CommandNumber::Experimentation.into();
        assert_eq!(forbidden, 1u8);
        assert_eq!(start_session, 2u8);
        assert_eq!(stop_session, 3u8);
        assert_eq!(request_tw_session, 5u8);
        assert_eq!(experimentation, 6u8);
    }
}
