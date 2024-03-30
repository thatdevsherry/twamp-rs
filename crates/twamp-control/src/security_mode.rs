#[derive(Debug, PartialEq)]
pub enum Mode {
    Abort = 0,
    UnAuthenticated = 1,
    Authenticated = 2,
    Encrypted = 4,
}
