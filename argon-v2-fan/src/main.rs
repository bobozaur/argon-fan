use std::error::Error;

use argon_fan::{ArgonCase, FanController};
use tracing_subscriber::EnvFilter;

/// Argon40 V2 case.
struct ArgonV2;

/// [`ArgonCase`] impl for [`ArgonV2`].
///
/// The speed gets set by accessing the equivalent command address.
impl ArgonCase for ArgonV2 {
    fn speed_command(speed: u8) -> (u8, u8) {
        (speed, 0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive("info".parse()?)
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    FanController::<ArgonV2>::new()?.run()?;
    Ok(())
}
