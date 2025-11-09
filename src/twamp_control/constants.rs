pub const TWAMP_CONTROL_WELL_KNOWN_PORT: u16 = 862;

#[derive(PartialEq)]
pub enum Messages {
    SetUpResponse,
    RequestTwSession,
    StartSessions,
    StopSessions,
}
