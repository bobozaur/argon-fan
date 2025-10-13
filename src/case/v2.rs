use crate::case::ArgonCase;

pub type Argon = ArgonV2;

/// Argon40 V2 case.
pub struct ArgonV2;

/// [`ArgonCase`] impl for [`ArgonV2`].
///
/// The speed gets set by accessing the equivalent command address.
impl ArgonCase for ArgonV2 {
    fn i2c_fan_command(speed: u8) -> (u8, u8) {
        (speed, 0)
    }
}
