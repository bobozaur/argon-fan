use thiserror::Error as ThisError;

use crate::{I2cError, IoError};

#[derive(Debug, ThisError)]
pub enum ControllerError {
    #[error("I2C error")]
    I2c(#[from] I2cError),
    #[error("temperature read error")]
    TempRead(#[source] IoError),
}
