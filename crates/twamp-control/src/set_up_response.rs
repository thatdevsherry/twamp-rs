#![allow(dead_code)]

use crate::security_mode::Mode;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetUpResponse {
    /// The mode that the client chooses to use during
    /// this TWAMP-Control session. It will also be used
    /// for all OWAMP-Test sessions started under control
    /// of this TWAMP-Control session.
    mode: Mode,

    #[serde(with = "BigArray")]
    key_id: [u8; 80],

    #[serde(with = "BigArray")]
    token: [u8; 64],

    client_iv: [u8; 16],
}

impl SetUpResponse {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::UnAuthenticated => SetUpResponse {
                mode,
                key_id: [0; 80],
                token: [0; 64],
                client_iv: [0; 16],
            },
            Mode::Abort => panic!("Mode 0, server don't wanna continue"),
            _ => panic!("Not supported"),
        }
    }
}
