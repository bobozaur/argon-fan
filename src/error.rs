pub use std::io::Error as IoError;

pub use rppal::i2c::Error as I2cError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ControllerError {
    #[error("I2C error")]
    I2c(#[from] I2cError),
    #[error("temperature read error")]
    TempRead(#[source] IoError),
}
