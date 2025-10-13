use std::time::Duration;

use rppal::i2c::I2c;
use systemstat::{Platform, System};
use tracing::instrument;

use crate::{
    case::{Argon, ArgonCase},
    config::FanCurvePoint,
    error::{ControllerError, I2cError, IoError},
};

#[allow(
    missing_debug_implementations,
    reason = "cannot auto-derive due to System"
)]
pub struct FanController {
    i2c: I2c,
    system: System,
    state: ControllerState,
    fan_curve: Vec<FanCurvePoint>,
    cooldown_cycles: u8,
    filter_factor: f32,
    current_speed: u8,
    prev_temp: f32,
}

impl FanController {
    const I2C_BUS: u8 = 1;
    const I2C_ADDRESS: u16 = 0x1a;

    /// Creates a new [`FanController`].
    ///
    /// # Errors
    /// Will error out if communicating with the I2C device or reading the CPU temperature fails.
    #[instrument(err(Debug))]
    pub fn new(
        fan_curve: Vec<FanCurvePoint>,
        cooldown_cycles: u8,
        filter_factor: f32,
    ) -> Result<Self, ControllerError> {
        tracing::info!("Creating fan controller...");

        tracing::info!("Connecting to I2C bus...");
        let mut i2c = I2c::with_bus(Self::I2C_BUS)?;

        tracing::info!("Connecting to I2C device...");
        // Set the I2C device address and sleep a bit to ensure it gets set.
        i2c.set_slave_address(Self::I2C_ADDRESS)?;
        std::thread::sleep(Duration::from_millis(100));

        tracing::debug!("Reading initial CPU temperature...");
        let system = System::new();
        let prev_temp = system.cpu_temp().map_err(ControllerError::TempRead)?;

        tracing::info!("Initial CPU temperature: {prev_temp}");

        let mut controller = Self {
            i2c,
            system: System::new(),
            state: ControllerState::Regular,
            fan_curve,
            cooldown_cycles,
            filter_factor,
            current_speed: 0,
            prev_temp,
        };

        // Start with the fan turned off.
        controller.set_speed(0)?;
        Ok(controller)
    }

    /// Runs the fan controller logic once. Meant to be called in a loop.
    ///
    /// # Errors
    /// Will error out if communicating with the I2C device or reading the CPU temperature fails.
    #[instrument(skip(self), err(Debug))]
    pub fn run_once(&mut self) -> Result<(), ControllerError> {
        let temp = self.read_temp().map_err(ControllerError::TempRead)?;

        let new_speed = self
            .fan_curve
            .iter()
            .rfind(|FanCurvePoint { temp: t, .. }| *t <= temp)
            .map(|FanCurvePoint { speed, .. }| *speed)
            .unwrap_or_default();

        match self.state {
            // If we have to ramp up the speed, do it.
            _ if new_speed > self.current_speed => {
                self.set_speed(new_speed)?;
                self.state = ControllerState::Regular;
            }
            // Cooldown is over so update to the new speed.
            ControllerState::Cooldown { cycles: 0 } => {
                self.set_speed(new_speed)?;
                self.state = ControllerState::Regular;
            }
            // Cooling down after a temperature increase.
            ControllerState::Cooldown { cycles } => {
                self.state = ControllerState::Cooldown { cycles: cycles - 1 };
            }
            // Avoid setting the same speed.
            ControllerState::Regular if new_speed == self.current_speed => {}
            // Speed is lower but no cooldown set; just update the speed.
            ControllerState::Regular if self.cooldown_cycles == 0 => self.set_speed(new_speed)?,
            // Speed is lower but cooldown first.
            ControllerState::Regular => {
                // This is considered the first cooldown cycle.
                let cycles = self.cooldown_cycles - 1;
                self.state = ControllerState::Cooldown { cycles };
            }
        }

        Ok(())
    }

    /// Reads and filters the CPU temperature using the filter factor.
    #[instrument(skip(self), err(Debug))]
    fn read_temp(&mut self) -> Result<f32, IoError> {
        tracing::debug!("Reading CPU temperature...");

        let prev_temp = self.prev_temp;
        let new_temp = self.system.cpu_temp()?;
        self.prev_temp = new_temp;

        let temp = new_temp * self.filter_factor + (1.0 - self.filter_factor) * prev_temp;
        tracing::info!("CPU temperature: prev={prev_temp}; new={new_temp}; filtered={temp}");

        Ok(temp)
    }

    /// Sets the fan speed percentage.
    #[instrument(skip(self), err(Debug))]
    fn set_speed(&mut self, speed: u8) -> Result<(), I2cError> {
        tracing::info!("Setting fan speed: {speed}%");

        let (command, value) = Argon::i2c_fan_command(speed);
        self.i2c.smbus_write_byte(command, value)?;
        self.current_speed = speed;

        Ok(())
    }
}

impl Drop for FanController {
    fn drop(&mut self) {
        tracing::info!("Fan controller shutting down...");

        self.set_speed(0)
            .inspect_err(|e| tracing::warn!("error turning off fan: {e}"))
            .ok();
    }
}

/// [`FanController`] state.
#[derive(Clone, Copy, Debug)]
enum ControllerState {
    /// The fan is running as usual.
    Regular,
    /// The fan is in cooldown mode, meaning that
    /// CPU temperature has dropped yet we want to keep it that way.
    ///
    /// Contains the remaining cycles left for cooldown.
    Cooldown { cycles: u8 },
}
