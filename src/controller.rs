use std::{marker::PhantomData, time::Duration};

use rppal::i2c::I2c;
use systemstat::{Platform, System};
use tracing::instrument;

use crate::{
    ArgonCase, I2cError, IoError,
    config::{Config, FanCurvePoint},
    error::ControllerError,
};

#[allow(
    missing_debug_implementations,
    reason = "cannot auto-derive due to System"
)]
pub struct FanController<C: ArgonCase> {
    case: PhantomData<fn() -> C>,
    config: Config,
    i2c: I2c,
    system: System,
    state: ControllerState,
    current_speed: u8,
    prev_temp: f32,
}

impl<C> FanController<C>
where
    C: ArgonCase,
{
    const I2C_BUS: u8 = 1;
    const I2C_ADDRESS: u16 = 0x1a;

    /// Creates a new [`FanController`].
    ///
    /// # Errors
    /// Will error out if parsing the config file,
    /// connecting to the I2C bus or reading the CPU temperature fails.
    #[instrument(err(Debug))]
    pub fn new() -> Result<Self, ControllerError> {
        tracing::info!("Creating fan controller...");

        let config = Config::new()?;

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
            case: PhantomData,
            config,
            i2c,
            system: System::new(),
            state: ControllerState::Regular,
            current_speed: 0,
            prev_temp,
        };

        controller.set_speed(0)?;
        Ok(controller)
    }

    /// Runs the fan controller loop.
    ///
    /// # Errors
    /// Will error out if communicating with the I2C device or reading the CPU temperature fails.
    #[instrument(skip(self), err(Debug))]
    pub fn run(mut self) -> Result<(), ControllerError> {
        loop {
            let temp = self.read_temp().map_err(ControllerError::TempRead)?;

            let new_speed = self
                .config
                .fan_curve
                .iter()
                .rfind(|FanCurvePoint { temp: t, .. }| *t <= temp)
                .map(|FanCurvePoint { speed, .. }| *speed)
                .unwrap_or_default();

            match self.state {
                _ if new_speed > self.current_speed => {
                    self.set_speed(new_speed)?;
                    self.current_speed = new_speed;
                    self.state = ControllerState::Regular;
                }

                ControllerState::Regular if new_speed == self.current_speed => {}
                ControllerState::Regular => {
                    self.state = ControllerState::Cooldown {
                        cycles: self.config.cooldown_cycles,
                    };
                    continue;
                }
                ControllerState::Cooldown { cycles: 0 } => {
                    self.state = ControllerState::Regular;
                    self.current_speed = 0;
                    continue;
                }
                ControllerState::Cooldown { cycles } => {
                    self.state = ControllerState::Cooldown { cycles: cycles - 1 };
                }
            }

            std::thread::sleep(Duration::from_secs(self.config.poll_interval_secs));
        }
    }

    #[instrument(skip(self), err(Debug))]
    fn read_temp(&mut self) -> Result<f32, IoError> {
        tracing::debug!("Reading CPU temperature...");

        let prev_temp = self.prev_temp;
        let new_temp = self.system.cpu_temp()?;
        self.prev_temp = new_temp;

        let factor = self.config.filter_factor;
        let temp = new_temp * factor + (1.0 - factor) * self.prev_temp;
        tracing::info!("CPU temperature: prev={prev_temp}; new={new_temp}; filtered={temp}");

        Ok(temp)
    }

    #[instrument(skip(self), err(Debug))]
    fn set_speed(&mut self, speed: u8) -> Result<(), I2cError> {
        tracing::info!("Setting fan speed: {speed}%");

        let (command, value) = C::speed_command(speed);
        self.i2c.smbus_write_byte(command, value)?;
        self.current_speed = speed;

        Ok(())
    }
}

impl<C> Drop for FanController<C>
where
    C: ArgonCase,
{
    fn drop(&mut self) {
        self.set_speed(0)
            .inspect_err(|e| tracing::warn!("error turning off fan: {e}"))
            .ok();
    }
}
#[derive(Clone, Copy, Debug)]
enum ControllerState {
    Regular,
    Cooldown { cycles: u8 },
}
