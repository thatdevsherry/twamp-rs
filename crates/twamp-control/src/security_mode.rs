use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[repr(u32)]
pub enum Mode {
    Abort = 0u32,
    #[default]
    UnAuthenticated = 1u32,
    Authenticated = 2u32,
    Encrypted = 4u32,
}
