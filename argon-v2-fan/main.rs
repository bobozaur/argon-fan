use std::error::Error;

use argon_fan::ArgonCase;

/// Argon40 V2 case.
struct ArgonV2;

/// [`ArgonCase`] impl for [`ArgonV2`].
///
/// The speed gets set by accessing the equivalent command address.
impl ArgonCase for ArgonV2 {
    fn i2c_fan_command(speed: u8) -> (u8, u8) {
        (speed, 0)
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    argon_fan::run::<ArgonV2>()
}
