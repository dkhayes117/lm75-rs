//! This is a platform agnostic Rust driver for the LM75 temperature
//! sensor and thermal watchdog, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Read the temperature.
//! - Set the fault queue.
//! - Set the OS temperature.
//! - Set the hysteresis temperature.
//! - Set the OS operation mode.
//! - Set the OS polarity.
//!
//! ## The device
//!
//! The LM75 temperature sensor includes a delta-sigma analog-to-digital
//! converter, and a digital overtemperature detector. The host can
//! query the LM75 through its I2C interface to read temperature at any
//! time. The open-drain overtemperature output (OS) sinks current when
//! the programmable temperature limit is exceeded.
//! The OS output operates in either of two modes, comparator or
//! interrupt. The host controls the temperature at which the alarm is
//! asserted (TOS) and the hysteresis temperature below which the alarm
//! condition is not valid (THYST). Also, the LM75's TOS and THYST
//! registers can be read by the host. The address of the LM75 is set
//! with three pins to allow multiple devices to work on the same bus.
//! Power-up is in comparator mode, with defaults of TOS= +80ºC and
//! THYST= +75ºC. The 3.0V to 5.5V supply voltage range, low supply
//! current, and I2C interface make the LM75 ideal for many applications
//! in thermal management and protection.
//!
//! Datasheet:
//! - [LM75](https://datasheets.maximintegrated.com/en/ds/LM75.pdf)
//!
//! This driver is also compatible with at least [LM75A], [LM75B, LM75C],
//! [AT30TS75A], [DS1775], [DS75], [DS7505], [G751], [MAX7500/1/2/3/4],
//! [MAX6625], [MCP9800/1/2/3], [STDS75], [TCN75].
//!
//! [AT30TS75A]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8839-DTS-AT30TS75A-Datasheet.pdf
//! [DS1775]: https://datasheets.maximintegrated.com/en/ds/DS1775-DS1775R.pdf
//! [DS75]: https://datasheets.maximintegrated.com/en/ds/DS75.pdf
//! [DS7505]: https://datasheets.maximintegrated.com/en/ds/DS7505.pdf
//! [G751]: http://www.gmt.com.tw/product/datasheet/EDS-751.pdf
//! [LM75A]: https://www.nxp.com/docs/en/data-sheet/LM75A.pdf
//! [LM75B, LM75C]: http://www.ti.com/lit/ds/symlink/lm75b.pdf
//! [MAX6625]: https://datasheets.maximintegrated.com/en/ds/MAX6625-MAX6626.pdf
//! [MAX7500/1/2/3/4]: https://datasheets.maximintegrated.com/en/ds/MAX7500-MAX7504.pdf
//! [MCP9800/1/2/3]: http://ww1.microchip.com/downloads/en/DeviceDoc/21909d.pdf
//! [STDS75]: https://www.st.com/resource/en/datasheet/stds75.pdf
//! [TCN75]: http://ww1.microchip.com/downloads/en/DeviceDoc/21490D.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read temperature
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Lm75::new(dev, address);
//! let temp_celsius = sensor.read_temperature().unwrap();
//! println!("Temperature: {}ºC", temp_celsius);
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut sensor = Lm75::new(dev, address);
//! ```
//!
//! ### Set the fault queue
//!
//! This is the number of consecutive faults necessary to trigger
//! an OS condition.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr, FaultQueue};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_fault_queue(FaultQueue::_4).unwrap();
//! ```
//!
//! ### Set the OS polarity
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr, OsPolarity};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_os_polarity(OsPolarity::ActiveHigh).unwrap();
//! ```
//!
//! ### Set the OS operation mode
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr, OsMode};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_os_mode(OsMode::Interrupt).unwrap();
//! ```
//!
//! ### Set the OS temperature
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! let temp_celsius = 50.0;
//! sensor.set_os_temperature(temp_celsius).unwrap();
//! ```
//!
//! ### Set the hysteresis temperature
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! let temp_celsius = 40.0;
//! sensor.set_hysteresis_temperature(temp_celsius).unwrap();
//! ```
//!
//! ### Enable / disable the sensor
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use lm75::{Lm75, SlaveAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.disable().unwrap(); // shutdown
//! sensor.enable().unwrap();
//! ```

#![deny(missing_docs, unsafe_code)]
#![no_std]

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
    /// Register is not implemented
    InvalidRegister,
}

/// Possible address pin states
#[derive(Debug, Clone)]
pub enum PinState {
    /// Logical Low Pin State
    Low,
    /// Logical High Pin State
    High,
    /// Logical Floating Pin State
    Floating,
}

/// I2C device address
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Address(pub(crate) u8);

/// Default device
impl Default for Address {
    fn default() -> Self {
        Address(DEVICE_BASE_ADDRESS)
    }
}

/// Support custom (integer) addresses
impl From<u8> for Address {
    fn from(a: u8) -> Self {
        Address(a)
    }
}

/// Compute device address from address bits where bits are not floating
impl From<(bool, bool, bool)> for Address {
    fn from(a: (bool, bool, bool)) -> Self {
        Address(
            DEVICE_BASE_ADDRESS
                | ((a.0 as u8) << 2)
                | ((a.1 as u8) << 1)
                | a.2 as u8,
        )
    }
}

/// Fault queue
///
/// Number of consecutive faults necessary to trigger OS condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultQueue {
    /// 1 fault will trigger OS condition (default)
    _1,
    /// 2 consecutive faults will trigger OS condition
    _2,
    /// 4 consecutive faults will trigger OS condition
    _4,
    /// 6 consecutive faults will trigger OS condition
    _6,
}

/// OS polarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OsPolarity {
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh,
}

/// OS operation mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OsMode {
    /// Comparator (default)
    Comparator,
    /// Interrupt
    Interrupt,
}

#[derive(Debug, Clone, Copy)]
/// Device Resolution
pub enum Resolution {
    /// 9bit has 0.5 resolution, 11bit has 0.125
    /// Masks the LSB only
    Mask9bit = 0b1000_0000,
    /// Sensors with 11-bit resolution (PCT2075)
    Mask11bit = 0b1110_0000,
}

impl Default for Resolution {
    fn default() -> Self { Resolution::Mask9bit }
}

const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

#[derive(Debug, Clone, Copy)]
struct Config {
    bits: u8,
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SampleRate {
    bits: Option<u8>,
}

impl Default for SampleRate {
    fn default() -> Self { SampleRate { bits: Some(1) } }
}

impl SampleRate {
    fn none() -> Self { SampleRate { bits: None } }
}

/// LM75 device driver.
#[derive(Debug, Default)]
pub struct Lm75<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: Config,
    /// Device Resolution
    resolution: Resolution,
    /// T-Idle Register Contents
    sample_rate: SampleRate,
}

mod conversion;
mod device_impl;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_address_matches_alternative_all_false() {
        assert_eq!(
            Address::default(),
            Address::from((false, false, false))
        )
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            Address::from(0b100_1000),
            Address::from((false, false, false))
        );
        assert_eq!(
            Address::from(0b100_1001),
            Address::from((false, false, true))
        );
        assert_eq!(
            Address::from(0b100_1010),
            Address::from((false, true, false))
        );
        assert_eq!(
            Address::from(0b100_1100),
            Address::from((true, false, false))
        );
        assert_eq!(
            Address::from(0b100_1111),
            Address::from((true, true, true))
        );
    }
}
