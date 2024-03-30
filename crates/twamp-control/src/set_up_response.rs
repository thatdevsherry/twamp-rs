#![allow(dead_code)]

use crate::security_mode::Mode;

#[derive(Debug)]
pub struct SetUpResponse {
    mode: Mode,
    key_id: [u8; 80],
    token: [u8; 64],
    client_iv: [u8; 16],
}

impl SetUpResponse {
    fn new(mode: Mode) -> Self {
        match mode {
            Mode::UnAuthenticated | Mode::Abort => SetUpResponse {
                mode,
                key_id: [0; 80],
                token: [0; 64],
                client_iv: [0; 16],
            },
            _ => panic!("Not supported"),
        }
    }
}
