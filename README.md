# Argon Fan

Rust binaries and Home Assistant add-ons for controlling fans in Argon40 Raspberry Pi cases.

## Overview

The binary uses a generic interface for controlling Argon40 case fans via I2C, with specific implementations for different case versions that handle their I2C command particularities. The case version/variant must be selected at compile time through a feature flag.

## Features

- Temperature-based fan curve control
- Configurable polling intervals and cooldown cycles
- Temperature filtering for smooth fan operation
- Support for both TOML and JSON configuration (feature gated)
- Systemd service integration
- Home Assistant add-on support

## Configuration

The fan control behavior is configured via TOML or JSON files (feature gated).
By default the `/etc/argon-fan/config.toml` file is used.

A different config file can be passed to the binaries as an argument:

```bash
argon-fan --config /path/to/config
```

Example configuration:

```toml
# Polling interval in seconds
poll_interval_secs = 1

# Cooldown cycles to keep fan speed stable
cooldown_cycles = 5

# Temperature filtering factor (0.0 to 1.0)
filter_factor = 0.2

# Fan curve points (temperature -> speed percentage)
fan_curve = [
    { temp = 30, speed = 40 },
    { temp = 65, speed = 100 }
]
```

## Installation

### From Source

Install the cross-compilation tools:

```bash
cargo install cross cargo-deb
```

Build DEB packages for Raspberry Pi (ARM64):

```bash
# For Argon V2 cases
cross build --release --features v2 --target aarch64-unknown-linux-gnu
cargo deb --target aarch64-unknown-linux-gnu --no-build --variant=v2

# For Argon V3 cases
cross build --release --features v3 --target aarch64-unknown-linux-gnu
cargo deb --target aarch64-unknown-linux-gnu --no-build --variant=v3
```

The `.deb` packages will be generated in `target/aarch64-unknown-linux-gnu/debian/` and can be installed on your Raspberry Pi.

### Home Assistant Add-on

This repository also serves Home Assistant add-ons.

## Requirements

- Raspberry Pi with I2C enabled (see [HassOSConfigurator](https://github.com/adamoutler/HassOSConfigurator))

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.
