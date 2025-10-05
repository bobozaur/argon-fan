use std::error::Error;

use argon_fan::{ArgonCase, FanController};
use tracing_subscriber::EnvFilter;

/// Argon40 V3 case.
struct ArgonV3;

/// [`ArgonCase`] impl for [`ArgonV3`].
///
/// The fan has a dedicated I2C command address to which the speed get sent.
impl ArgonCase for ArgonV3 {
    fn speed_command(speed: u8) -> (u8, u8) {
        // I2C fan command address.
        const FAN_COMMAND: u8 = 0x80;
        (FAN_COMMAND, speed)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive("info".parse()?)
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    FanController::<ArgonV3>::new()?.run()?;
    Ok(())
}
