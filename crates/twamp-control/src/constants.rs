// Replace with 862, using 4000 since it doesn't require permission stuff
pub const TWAMP_CONTROL_WELL_KNOWN_PORT: u16 = 4000;

pub enum Messages {
    SetUpResponse,
    RequestTwSession,
    StartSessions,
    StopSessions,
}
