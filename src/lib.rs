#![cfg_attr(not(test), no_std)]

//! Driver for the iC-MD quadrature counter.
//! Built fully in Rust, uses [embedded_hal] and [device_driver].

use core::{fmt::Debug, result::Result};
use embedded_hal::spi::SpiDevice;

use dd::{Device, DeviceError, DeviceInterface};

pub use dd::CntCfg;

pub mod dd;

/// Represent the counter values for different configurations of the iC-MD quadrature counter.
/// If more than one counter value is present, the counter values are always in the order of
/// Counter 0, Counter 1, and Counter 2.
/// Note: The size of the returned value depends on the configuration of the counter!
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CounterCount {
    Cnt1Bit24(i32),
    Cnt2Bit24(i32, i32),
    Cnt1Bit48(i64),
    Cnt1Bit16(i16),
    Cnt1Bit32(i32),
    Cnt2Bit32Bit16(i16, i32),
    Cnt2Bit16(i16, i16),
    Cnt3Bit16(i16, i16, i16),
}

/// The main driver struct of the crate representing the iC-MD quadrature counter.
#[derive(Debug)]
pub struct IcMd<Spi> {
    device: Device<DeviceInterface<Spi>>,
    config: CntCfg,
}

impl<Spi: SpiDevice> IcMd<Spi> {
    /// Creates a new instance of the iC-MD driver.
    /// By default, the counter is configured to 48-bit mode.
    pub fn new(spi: Spi) -> Self {
        let device = Device::new(DeviceInterface::new(spi));
        let config = CntCfg::Cnt1Bit48;
        Self { device, config }
    }

    pub fn init(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        self.device
            .counter_configuration()
            .write(|reg| reg.set_value(self.config))?;
        Ok(())
    }

    /// Read the current counter value and return it.
    pub fn read_counter(&mut self) -> Result<CounterCount, DeviceError<Spi::Error>> {
        match self.config {
            CntCfg::Cnt1Bit24 => Ok(CounterCount::Cnt1Bit24(
                self.device.read_cnt_cfg_0().read()?.cnt_0(),
            )),
            CntCfg::Cnt2Bit24 => Ok(CounterCount::Cnt2Bit24(
                self.device.read_cnt_cfg_1().read()?.cnt_0(),
                self.device.read_cnt_cfg_1().read()?.cnt_1(),
            )),
            CntCfg::Cnt1Bit48 => Ok(CounterCount::Cnt1Bit48(
                self.device.read_cnt_cfg_2().read()?.cnt_0(),
            )),
            CntCfg::Cnt1Bit16 => Ok(CounterCount::Cnt1Bit16(
                self.device.read_cnt_cfg_3().read()?.cnt_0(),
            )),
            CntCfg::Cnt1Bit32 => Ok(CounterCount::Cnt1Bit32(
                self.device.read_cnt_cfg_4().read()?.cnt_0(),
            )),
            CntCfg::Cnt2Bit32Bit16 => Ok(CounterCount::Cnt2Bit32Bit16(
                self.device.read_cnt_cfg_5().read()?.cnt_0(),
                self.device.read_cnt_cfg_5().read()?.cnt_1(),
            )),
            CntCfg::Cnt2Bit16 => Ok(CounterCount::Cnt2Bit16(
                self.device.read_cnt_cfg_6().read()?.cnt_0(),
                self.device.read_cnt_cfg_6().read()?.cnt_1(),
            )),
            CntCfg::Cnt3Bit16 => Ok(CounterCount::Cnt3Bit16(
                self.device.read_cnt_cfg_7().read()?.cnt_0(),
                self.device.read_cnt_cfg_7().read()?.cnt_1(),
                self.device.read_cnt_cfg_7().read()?.cnt_2(),
            )),
        }
    }
}
