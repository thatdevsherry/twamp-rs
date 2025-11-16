/// Well-known port for [TWAMP-Control](super) as per [RFC 8545](https://datatracker.ietf.org/doc/html/rfc8545#section-7).
pub const TWAMP_CONTROL_WELL_KNOWN_PORT: u16 = 862;

#[derive(Debug, PartialEq, Eq)]
pub enum ControlMessages {
    ServerGreeting,
    SetUpResponse,
    ServerStart,
    RequestTwSession,
    AcceptSession,
    StartSessions,
    StartAck,
    StopSessions,
}
