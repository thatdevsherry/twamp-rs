use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimeStamp {
    pub integer_part_of_seconds: [u8; 4],
    pub fractional_part_of_seconds: [u8; 4],
}

impl TimeStamp {
    pub fn new() -> Self {
        TimeStamp {
            integer_part_of_seconds: [0; 4],
            fractional_part_of_seconds: [0; 4],
        }
    }
}
