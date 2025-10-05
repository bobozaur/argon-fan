mod controller;
mod error;
mod config;

use std::io::Error as IoError;

use rppal::i2c::Error as I2cError;

pub use crate::{controller::FanController, error::ControllerError};

pub trait ArgonCase {
    fn speed_command(speed: u8) -> (u8, u8);
}
