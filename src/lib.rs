mod config;
mod controller;
mod error;

use std::{
    error::Error,
    io::Error as IoError,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use clap::Parser;
use rppal::i2c::Error as I2cError;
use signal_hook::consts::{SIGINT, SIGQUIT, SIGTERM};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{
    config::{Args, Config},
    controller::FanController,
};

/// Trait for abstracting over `Argon40` case versions.
pub trait ArgonCase {
    /// Method used for determining the I2C fan speed command address and data payload.
    /// Return a tuple of (`command address`, `data byte`).
    fn i2c_fan_command(speed: u8) -> (u8, u8);
}

/// Daemon entrypoint that:
/// - sets up the logger
/// - parses CLI arguments
/// - reads the config file
/// - sets up signal handlers for (SIGTERM, SIGINT and SIGQUIT)
/// - runs the fan controller, sleeping for the configured interval between cycles
///
/// While doing all of these from within a library is somewhat unorthodox, this library
/// is meant to be the common ground between the daemons with the only variation being the
/// targeted case.
///
/// # Errors
/// Errors out if any of the above fails.
pub fn run<C>() -> Result<(), Box<dyn Error + Send + Sync>>
where
    C: ArgonCase,
{
    // Setup the logger
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()?;

    // Read CLI args
    let path = Args::try_parse()?.config;

    // Read config file
    let Config {
        poll_interval_secs,
        cooldown_cycles,
        filter_factor,
        fan_curve,
    } = Config::new(&path)?;

    // Setup signal handlers
    let should_stop = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM, should_stop.clone())?;
    signal_hook::flag::register(SIGINT, should_stop.clone())?;
    signal_hook::flag::register(SIGQUIT, should_stop.clone())?;

    // Setup the fan controller
    let mut fan_controller = FanController::<C>::new(fan_curve, cooldown_cycles, filter_factor)?;

    // Run until a signal is received.
    while !should_stop.load(Ordering::Relaxed) {
        fan_controller.run_once()?;
        std::thread::sleep(Duration::from_secs(poll_interval_secs));
    }

    Ok(())
}
