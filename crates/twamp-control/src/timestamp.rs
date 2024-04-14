use crate::constants::NTP_EPOCH;
use deku::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// See [RFC 1305](https://datatracker.ietf.org/doc/html/rfc1305) for the format.
#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TimeStamp {
    integer_part_of_seconds: u32,
    fractional_part_of_seconds: u32,
}

impl Default for TimeStamp {
    /// Create using current time.
    fn default() -> Self {
        let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self::from(unix_time)
    }
}

impl From<Duration> for TimeStamp {
    /// Convert from a Duration.
    ///
    /// **Note** that it assumes the duration is from [`UNIX_EPOCH`].
    ///
    /// It performs conversion from [`UNIX_EPOCH`] duration to [`NTP_EPOCH`] duration.
    fn from(value: Duration) -> Self {
        // Safe to use unwrap since `UNIX_EPOCH` will **always** be earlier than `now()`.
        let now_since_ntp_epoch = value + Duration::from_secs(NTP_EPOCH);
        let integer_part = now_since_ntp_epoch.as_secs() % 4_294_967_296u64;
        let fractional_part = now_since_ntp_epoch.subsec_nanos();

        Self {
            integer_part_of_seconds: integer_part as u32,
            fractional_part_of_seconds: fractional_part,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn timestamp_from_duration() {
        let duration = Duration::from_nanos(1713088089243932687);
        let integer_part = duration.as_secs();
        let fractional_part = duration.subsec_nanos();
        let timestamp = TimeStamp::from(duration);
        assert_eq!(
            timestamp.integer_part_of_seconds,
            (integer_part + NTP_EPOCH) as u32
        );
        assert_eq!(timestamp.fractional_part_of_seconds, fractional_part);
    }
}
