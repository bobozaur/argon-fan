mod case;
mod config;
mod controller;
mod error;

use std::{
    error::Error,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use clap::Parser;
use signal_hook::consts::{SIGINT, SIGQUIT, SIGTERM};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{
    config::{Args, Config},
    controller::FanController,
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
    let mut fan_controller = FanController::new(fan_curve, cooldown_cycles, filter_factor)?;

    // Run until a signal is received.
    while !should_stop.load(Ordering::Relaxed) {
        fan_controller.run_once()?;
        std::thread::sleep(Duration::from_secs(poll_interval_secs));
    }

    Ok(())
}
