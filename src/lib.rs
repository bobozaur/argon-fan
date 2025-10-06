mod config;
mod controller;
mod error;

use std::io::Error as IoError;

use rppal::i2c::Error as I2cError;

pub use crate::{controller::FanController, error::ControllerError};

/// Trait for abstracting over `Argon40` case versions.
pub trait ArgonCase {
    /// Method used for determining the I2C fan speed command address and data payload.
    /// Return a tuple of (`command address`, `data byte`).
    fn i2c_fan_command(speed: u8) -> (u8, u8);
}
