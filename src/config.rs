use clap::Parser;
use config::{ConfigError, File};
use serde::Deserialize;
use tracing::instrument;

/// CLI arguments parser struct.
#[derive(Debug, Parser)]
#[command(name = "argon-fan", version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "/etc/argon-fan/config.toml")]
    pub config: String,
}

/// Service configuration struct.
#[derive(Debug, Deserialize)]
#[serde(try_from = "ConfigDe")]
pub struct Config {
    /// Polling interval in seconds.
    pub poll_interval_secs: u64,
    /// The number of cooldown cycles to keep the fan speed stable for.
    pub cooldown_cycles: u8,
    /// The filtering factor for smoothing CPU temperature.
    /// Must be between `0.0` and `1.0`.
    pub filter_factor: f32,
    /// Fan curve points array in strictly asceding order.
    pub fan_curve: Vec<FanCurvePoint>,
}

impl Config {
    #[instrument(ret, err(Debug))]
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        tracing::info!("Reading config from {path}...");

        config::Config::builder()
            .add_source(File::with_name(path))
            .build()?
            .try_deserialize()
    }
}

/// A fan curve point.
#[derive(Debug, Deserialize)]
#[serde(from = "FanCurvePointDe")]
pub struct FanCurvePoint {
    /// Temperature where the fan setting should trigger.
    pub temp: f32,
    /// Speed that the fan should be set to when the temperature is exceeded.
    pub speed: u8,
}

impl TryFrom<ConfigDe> for Config {
    type Error = ConfigError;

    fn try_from(value: ConfigDe) -> Result<Self, Self::Error> {
        let ConfigDe {
            poll_interval_secs,
            cooldown_cycles,
            filter_factor,
            fan_curve,
        } = value;

        if !(0.0..=1.0).contains(&filter_factor) {
            let msg = "filter_factor: must be between 0.0 and 1.0".to_owned();
            return Err(ConfigError::Message(msg));
        }

        if !fan_curve.is_sorted_by(|fc1, fc2| fc1.temp < fc2.temp && fc1.speed < fc2.speed) {
            let msg = "fan_curve: temperatures and speeds must both be increasing".to_owned();
            return Err(ConfigError::Message(msg));
        }

        Ok(Self {
            poll_interval_secs: poll_interval_secs.into(),
            cooldown_cycles,
            filter_factor,
            fan_curve,
        })
    }
}

impl From<FanCurvePointDe> for FanCurvePoint {
    fn from(value: FanCurvePointDe) -> Self {
        Self {
            temp: value.temp.into(),
            speed: value.speed,
        }
    }
}

/// Deserialization helper.
/// It helps with implicit config validation through the [`TryFrom`] conversion.
#[derive(Debug, Deserialize)]
struct ConfigDe {
    poll_interval_secs: u8,
    cooldown_cycles: u8,
    filter_factor: f32,
    fan_curve: Vec<FanCurvePoint>,
}

/// Deserialization helper.
/// It aids in converting `i8` temperature thresholds into `f32` for easier comparisons.
#[derive(Debug, Deserialize)]
struct FanCurvePointDe {
    temp: i8,
    speed: u8,
}
