use deku::prelude::*;

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct ErrorEstimate {
    /// SHOULD be set if the party generating the timestamp has a clock that is synchronized to UTC
    /// using an external source (e.g., the bit should be set if GPS hardware is used and it
    /// indicates that it has acquired current position and time or if NTP is used and it indicates
    /// that it has synchronized to an external source, which includes stratum 0 source, etc.).
    /// If there is no notion of external synchronization for the time source, the bit SHOULD NOT
    /// be set.
    #[deku(bits = "1")]
    s: u8,

    /// Same semantics as MBZ fields elsewhere: it MUST be set to zero by the sender and ignored
    /// by everyone else.
    #[deku(bits = "1", assert_eq = "0u8")]
    mbz: u8,

    /// An unsigned integer.
    #[deku(bits = "6")]
    scale: u8,

    /// Multiplier is an unsigned integer as well. They are interpreted
    /// as follows: the error estimate is equal to
    /// Multiplier*2^(-32)*2^Scale (in seconds).
    /// (Notation clarification: 2^Scale is two to the power of Scale.)
    /// Multiplier MUST NOT be set to zero.
    /// If Multiplier is zero, the packet SHOULD be considered corrupt and discarded.
    multiplier: u8,
}

impl ErrorEstimate {
    pub fn new(ntp_synchronized: bool) -> ErrorEstimate {
        ErrorEstimate {
            s: if ntp_synchronized { 1 } else { 0 },
            mbz: 0,
            scale: if ntp_synchronized { 0 } else { 63 },
            multiplier: if ntp_synchronized { 1 } else { 255 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_error_estimate_with_ntp_synchronized() {
        let error_estimate = ErrorEstimate::new(true);
        assert_eq!(error_estimate.s, 1);
        assert_eq!(error_estimate.mbz, 0);
        assert_eq!(error_estimate.scale, 0);
        assert_eq!(error_estimate.multiplier, 1);
    }

    #[test]
    fn create_error_estimate_with_ntp_not_synchronized() {
        let error_estimate = ErrorEstimate::new(false);
        assert_eq!(error_estimate.s, 0);
        assert_eq!(error_estimate.mbz, 0);
        assert_eq!(error_estimate.scale, 63);
        assert_eq!(error_estimate.multiplier, 255);
    }
}
