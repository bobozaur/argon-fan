use std::error::Error;

use argon_fan::ArgonCase;

/// Argon40 V3 case.
struct ArgonV3;

/// [`ArgonCase`] impl for [`ArgonV3`].
///
/// The fan has a dedicated I2C command address to which the speed get sent.
impl ArgonCase for ArgonV3 {
    fn i2c_fan_command(speed: u8) -> (u8, u8) {
        // I2C fan command address.
        const FAN_COMMAND: u8 = 0x80;
        (FAN_COMMAND, speed)
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    argon_fan::run::<ArgonV3>()
}
