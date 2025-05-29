#![cfg_attr(not(test), no_std)]

//! Driver for the iC-MD quadrature counter.
//! Built fully in Rust, uses [embedded_hal] and [device_driver].

use core::{fmt::Debug, result::Result};
use embedded_hal::spi::SpiDevice;

use dd::{Device, DeviceError, DeviceInterface};

pub use configs::*;
pub use dd::CntCfg;

pub mod configs;
pub mod dd;

/// The main driver struct of the crate representing the iC-MD quadrature counter.
/// You can also access the underlying device driver directly via the `device` field.
/// You are then yourself responsible for reading the correct counter configurations.
#[derive(Debug)]
pub struct IcMd<Spi> {
    pub device: Device<DeviceInterface<Spi>>,
    counter_config: CntCfg,
    device_status: DeviceStatus,
    actuator_status: ActuatorStatus,
}

impl<Spi: SpiDevice> IcMd<Spi> {
    /// Creates a new instance of the iC-MD driver.
    /// By default, the counter is configured to 48-bit mode.
    pub fn new(spi: Spi) -> Self {
        Self {
            device: Device::new(DeviceInterface::new(spi)),
            counter_config: CntCfg::Cnt1Bit48,
            actuator_status: ActuatorStatus::default(),
            device_status: DeviceStatus::default(),
        }
    }

    /// Initialize the iC-MD device with the given configuration.
    pub fn init(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        self.device
            .counter_configuration()
            .write(|reg| reg.set_value(self.counter_config))?;
        Ok(())
    }

    /// Set the actuator pins output to the given status.
    /// Note that as far as the iC-MD is concerned, this status is "write only". Thus, there is no
    /// function available to read the current status of the actuator pins. However, the stored
    /// `actuator_status` variable will be updated according to what you set here.
    ///
    /// # Arguments
    /// * `act0`: The status of actuator pin 0 (ACT0).
    /// * `act1`: The status of actuator pin 1 (ACT1).
    pub fn configure_actuator_pins(
        &mut self,
        act0: &PinStatus,
        act1: &PinStatus,
    ) -> Result<(), DeviceError<Spi::Error>> {
        self.device.instruction_byte().write(|reg| {
            reg.set_act_0(act0.into());
            reg.set_act_1(act1.into());
        })?;
        self.actuator_status.act0 = *act0;
        self.actuator_status.act1 = *act1;
        Ok(())
    }

    /// Touch probe instruction
    /// Load touch probe 2 with touch probe 1 value and touch probe 1 wiht ABCNT value.
    pub fn touch_probe_instruction(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        let act0 = &self.actuator_status.act0;
        let act1 = &self.actuator_status.act1;
        self.device.instruction_byte().write(|reg| {
            reg.set_tp(true);
            reg.set_act_0(act0.into());
            reg.set_act_1(act1.into());
        })?;
        Ok(())
    }

    /// Read the current counter value and return it.
    pub fn read_counter(&mut self) -> Result<CntCount, DeviceError<Spi::Error>> {
        match self.counter_config {
            CntCfg::Cnt1Bit24 => {
                let res = self.device.read_cnt_cfg_0().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit24(res.cnt_0()))
            }
            CntCfg::Cnt2Bit24 => {
                let res = self.device.read_cnt_cfg_1().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit24(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt1Bit48 => {
                let res = self.device.read_cnt_cfg_2().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit48(res.cnt_0()))
            }
            CntCfg::Cnt1Bit16 => {
                let res = self.device.read_cnt_cfg_3().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit16(res.cnt_0()))
            }
            CntCfg::Cnt1Bit32 => {
                let res = self.device.read_cnt_cfg_4().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit32(res.cnt_0()))
            }
            CntCfg::Cnt2Bit32Bit16 => {
                let res = self.device.read_cnt_cfg_5().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit32Bit16(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt2Bit16 => {
                let res = self.device.read_cnt_cfg_6().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit16(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt3Bit16 => {
                let res = self.device.read_cnt_cfg_7().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt3Bit16(res.cnt_0(), res.cnt_1(), res.cnt_2()))
            }
        }
    }

    /// Reset counters to zero.
    /// You can select which counters should be set to zero using the specific arguments.
    ///
    /// TODO: Check statement for correctness.
    /// This routine does not check if the counter you are trying to reset is actually configered.
    /// If the requested counter is not configured, the reset bit will be sent, but this will
    /// simply be ignored by the counter.
    ///
    /// # Arguments
    /// * `cnt0`: If true, counter 0 is reset, else not.
    /// * `cnt1`: If true, counter 1 is reset, else not.
    /// * `cnt2`: If true, counter 2 is reset, else not.
    pub fn reset_counters(
        &mut self,
        cnt0: bool,
        cnt1: bool,
        cnt2: bool,
    ) -> Result<(), DeviceError<Spi::Error>> {
        let act0 = &self.actuator_status.act0;
        let act1 = &self.actuator_status.act1;
        self.device.instruction_byte().write(|reg| {
            reg.set_ab_res_0(cnt0);
            reg.set_ab_res_1(cnt1);
            reg.set_ab_res_2(cnt2);
            reg.set_act_0(act0.into());
            reg.set_act_1(act1.into());
        })?;
        Ok(())
    }

    /// Reset all counters.
    /// Can be used to send reset commands to all counters.
    ///
    /// TODO: Check that this does not need to actually take counter config into account!
    pub fn reset_all_counters(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        self.reset_counters(true, true, true)?;
        Ok(())
    }

    /// Set device status from two bools that were read and passed on to here.
    fn set_device_status(&mut self, warn_status: bool, err_status: bool) {
        self.device_status.warning = warn_status.into();
        self.device_status.error = err_status.into();
    }
}
