#[allow(unused_attributes, reason = "conditionally compiled module")]
#[cfg_attr(feature = "v2", path = "v2.rs")]
#[cfg_attr(feature = "v3", path = "v3.rs")]
mod case_variant;

pub use case_variant::Argon;

/// Trait for abstracting over `Argon40` case versions.
pub trait ArgonCase {
    /// Method used for determining the I2C fan speed command address and data payload.
    /// Return a tuple of (`command address`, `data byte`).
    fn i2c_fan_command(speed: u8) -> (u8, u8);
}
