mod constants;
pub use constants::NTP_EPOCH;
use deku::prelude::*;
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// See [RFC 1305](https://datatracker.ietf.org/doc/html/rfc1305) for the format.
#[derive(Clone, Copy, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct TimeStamp {
    integer_part_of_seconds: u32,
    fractional_part_of_seconds: u32,
}

impl Sum for TimeStamp {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            TimeStamp {
                integer_part_of_seconds: 0,
                fractional_part_of_seconds: 0,
            },
            |acc, x| acc + x,
        )
    }
}

impl Add for TimeStamp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (fractional_sum, fractional_carry) = self
            .fractional_part_of_seconds
            .overflowing_add(rhs.fractional_part_of_seconds);
        let integer_part_of_seconds =
            self.integer_part_of_seconds + rhs.integer_part_of_seconds + (fractional_carry as u32);
        let fractional_part_of_seconds = fractional_sum.wrapping_add(1);

        TimeStamp {
            integer_part_of_seconds,
            fractional_part_of_seconds,
        }
    }
}

impl Sub for TimeStamp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut integer_part_of_seconds = self.integer_part_of_seconds;
        let mut fractional_part_of_seconds = self.fractional_part_of_seconds;

        if self.fractional_part_of_seconds < rhs.fractional_part_of_seconds {
            integer_part_of_seconds -= 1;
            fractional_part_of_seconds += u32::MAX;
        }

        TimeStamp {
            integer_part_of_seconds: integer_part_of_seconds - rhs.integer_part_of_seconds,
            fractional_part_of_seconds: fractional_part_of_seconds - rhs.fractional_part_of_seconds,
        }
    }
}

impl From<TimeStamp> for f64 {
    fn from(value: TimeStamp) -> Self {
        value.integer_part_of_seconds as f64
            + (value.fractional_part_of_seconds as f64 / u32::MAX as f64)
    }
}

impl From<Duration> for TimeStamp {
    /// Convert from a Duration.
    ///
    /// **Note** that it assumes the duration is from [`UNIX_EPOCH`].
    ///
    /// It performs conversion from `UNIX_EPOCH` duration to [`NTP_EPOCH`] duration.
    fn from(value: Duration) -> Self {
        let now_since_ntp_epoch = value + Duration::from_secs(NTP_EPOCH);
        let integer_part = now_since_ntp_epoch.as_secs() % 4_294_967_296u64;
        let fractional_part = now_since_ntp_epoch.subsec_nanos();

        Self {
            integer_part_of_seconds: integer_part as u32,
            fractional_part_of_seconds: fractional_part,
        }
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "integer, fractional: {}, {}",
            self.integer_part_of_seconds, self.fractional_part_of_seconds
        )
    }
}

impl Default for TimeStamp {
    fn default() -> Self {
        let duration_since_unix_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Self::try_from(duration_since_unix_epoch).unwrap()
    }
}

impl TimeStamp {
    pub fn integer_part_of_seconds(&self) -> u32 {
        self.integer_part_of_seconds
    }

    /// Return the fractional part, which is stored as nanos.
    pub fn fractional_part_of_seconds(&self) -> u32 {
        self.fractional_part_of_seconds
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
        let timestamp = TimeStamp::try_from(duration).unwrap();
        assert_eq!(
            timestamp.integer_part_of_seconds,
            (integer_part + NTP_EPOCH) as u32
        );
        assert_eq!(timestamp.fractional_part_of_seconds, fractional_part);
    }

    #[test]
    fn subtraction_from_bigger_to_smaller() {
        let t1 = TimeStamp {
            integer_part_of_seconds: 10,
            fractional_part_of_seconds: 1_000_000_000,
        };
        let t2 = TimeStamp {
            integer_part_of_seconds: 8,
            fractional_part_of_seconds: 1_000_000_000,
        };
        let result = t1 - t2;
        assert_eq!(
            result,
            TimeStamp {
                integer_part_of_seconds: 2,
                fractional_part_of_seconds: 0
            }
        )
    }

    #[test]
    fn addition() {
        let ts1 = TimeStamp {
            integer_part_of_seconds: 1,
            fractional_part_of_seconds: 3_000_000_000,
        };

        let ts2 = TimeStamp {
            integer_part_of_seconds: 2,
            fractional_part_of_seconds: 2_500_000_000,
        };

        let result = ts1 + ts2;
        assert_eq!(
            result,
            TimeStamp {
                integer_part_of_seconds: 4,
                fractional_part_of_seconds: 1_205_032_705
            }
        )
    }
}
