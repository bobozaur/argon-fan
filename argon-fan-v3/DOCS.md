# Argon V3 Fan

Automatic temperature-based fan control for Argon40 V3 Raspberry Pi cases.

## Installation

1. Navigate to the Home Assistant Add-on Store
2. Add this repository URL to your add-on repositories
3. Install the "Argon V3 Fan" add-on
4. Configure the add-on options
5. Start the add-on

## Configuration

### Add-on Configuration

```yaml
poll_interval_secs: 1
cooldown_cycles: 5
filter_factor: 0.2
fan_curve:
  - temp: 30
    speed: 40
  - temp: 65
    speed: 100
```

### Configuration Options

| Option               | Default   | Description                                                      |
| -------------------- | --------- | ---------------------------------------------------------------- |
| `poll_interval_secs` | `1`       | How often to check temperature (seconds)                         |
| `cooldown_cycles`    | `5`       | Number of cycles to wait before changing fan speed               |
| `filter_factor`      | `0.2`     | Temperature smoothing factor (0.0-1.0, higher = more responsive) |
| `fan_curve`          | See below | Array of temperature/speed points                                |

### Fan Curve Configuration

The fan curve defines how fan speed responds to temperature changes. Each point contains:

- `temp`: Temperature in Celsius
- `speed`: Fan speed as percentage (0-100)

**Requirements:**

- Points must be ordered by ascending temperature AND speed

### Enable I2C

If I2C is not enabled, enable it via [HassOSConfigurator](https://github.com/adamoutler/HassOSConfigurator).
