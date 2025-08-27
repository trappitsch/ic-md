//! Driver for the iC-MD quadrature counter.
//! Built fully in Rust, uses [embedded_hal] and [device_driver].
//!
//! <div class="warning">
//!
//! **Important Note:**
//!
//! This driver is in active development and not yet feature complete. Please see the section below
//! on Limitations for more details. This driver is currently only available via `git`.
//!
//! Any comments are welcome!
//!
//! </div>
//!
//! # Introduction
//!
//! The `IcMd` struct provides a high-level interface to interact with the iC-MD quadrature
//! counter. However, you can also access the underlying device driver directly via the `device`
//! field. Please read the device driver documentation for more information on what to expect
//! when interfacing with the device driver directly.
//! This low-level access is a temporary solution until the high-level interface is fully
//! developed. When this well be the case is unclear. If you are interested in it, please let me
//! know and I'm happy to prioritize the high-level features that are interesting to you.
//!
//! # Limitations
//!
//! The following features are currently only accessible via the low-level interface:
//!
//! - Reference register readout: It is unclear if this currently works, see code comment.
//!
//! The following features are currently not yet implemented:
//!
//! - Differential or TTL inputs (Address 0x01, bit 7)
//! - Configuration to have Z signal clear counters 0 and/or 1 (Address 0x01, bits 5 and 6)
//! - Z signal configuration (Address 0x01, bits 3 and 4)
//! - Touch probe and AB registers (Address 0x01, bits 1 and 2)
//! - Differential input configuration selection (RS-422 (default) or LVDS) (Address 0x03, bit 7)
//!
//! # Example Usage
//!
//! ```rust
//! # use embedded_hal_mock::eh1::spi::{Mock, Transaction};
//! # use ic_md::IcMd;
//! # let expectations = [
//! #     Transaction::transaction_start(),
//! #     Transaction::write(0x00),
//! #     Transaction::write(0x02),
//! #     Transaction::transaction_end(),
//! #     Transaction::transaction_start(),
//! #     Transaction::write(0x80 | 0x08),
//! #     Transaction::read_vec(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, 0xC0]),
//! #     Transaction::transaction_end(),
//! # ];
//! // Initialize your SPIDevice, here we are mocking a device!
//! let mut spi_device = Mock::new(&expectations);
//!
//! // Get a handle to the counter with the default setup
//! let mut icmd = IcMd::new(&mut spi_device);
//!
//! // Initialize the counter
//! icmd.init().unwrap();
//!
//! // Read out the counter
//! let counter_value = icmd.read_counter().unwrap();
//!
//! // We can use the get counter methods to access the values. This will return an `Option`
//! // containing an `i64` value of the count (if the counter is setup, otherwise `None`).
//! let cnt_0 = counter_value
//!     .get_cnt0()
//!     .expect("Counter 0 should always be set up");
//!
//! assert_eq!(cnt_0, 42);
//!
//! // Last, let us ensure that there are no errors or warnings in the device status. We can use
//! // the `.is_ok()` method on the `DeviceStatus` struct to do this.
//! assert!(icmd.get_device_status().is_ok());
//! #  
//! # // Check that all our expectations are met - testing only
//! # spi_device.done();
//! ```
//!
//! # Further help
//!
//! For further help and examples, please have a look at the `test` directory in the GitHub
//! repository, which you can find [here](https://github.com/trappitsch/ic-md/).
//! There you will find various integration tests that show how to use the driver in practice and
//! that contain detailed comments on for you.

#![deny(warnings, missing_docs)]
#![cfg_attr(not(test), no_std)]

use core::{fmt::Debug, result::Result};
use embedded_hal::spi::SpiDevice;

use dd::{Device, DeviceError, DeviceInterface};

pub use configs::*;

pub mod configs;
pub mod dd;

/// The main driver struct of the crate representing the iC-MD quadrature counter.
/// You can also access the underlying device driver directly via the `device` field.
/// You are then yourself responsible for reading the correct counter configurations.
#[derive(Debug)]
pub struct IcMd<Spi> {
    /// Provides acces to the underlying device driver.
    pub device: Device<DeviceInterface<Spi>>,
    /// Configuration of the counter, set only prior to calling `init()`.
    counter_config: CntCfg,
    /// Status of the device (error and warning flags). Read only, updated when reading the
    /// counter.
    device_status: DeviceStatus,
    actuator_status: ActuatorStatus,
}

impl<Spi: SpiDevice> IcMd<Spi> {
    /// Creates a new instance of the iC-MD driver.
    /// By default, the counter is configured to 48-bit mode.
    pub fn new(spi: Spi) -> Self {
        Self {
            device: Device::new(DeviceInterface::new(spi)),
            counter_config: CntCfg::Cnt1Bit48(CntSetup::default()),
            actuator_status: ActuatorStatus::default(),
            device_status: DeviceStatus::default(),
        }
    }

    /// Initialize the iC-MD device with the given configuration.
    pub fn init(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        self.device
            .counter_configuration()
            .write(|reg| reg.set_value(self.counter_config.into()))?;

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

    /// Get current device status.
    /// This is a cached value that is updated when reading the counter. It contains the error and
    /// warning flags of the device. For a full device status, use `get_full_device_status()`.
    pub fn get_device_status(&self) -> DeviceStatus {
        self.device_status
    }

    /// Get the full device status by reading all the status registers.
    /// This will reset many of the status bits to wait for the next event, problem, issue to
    /// occur.
    pub fn get_full_device_status(&mut self) -> Result<FullDeviceStatus, DeviceError<Spi::Error>> {
        let status0 = self.device.status_0().read()?;
        let status1 = self.device.status_1().read()?;
        let status2 = self.device.status_2().read()?;

        Ok(FullDeviceStatus {
            cnt0_overflow: status0.ovf_0().into(),
            cnt0_aberr: status0.ab_err_0().into(),
            cnt0_zero: status0.zero_0().into(),
            cnt1_overflow: status1.ovf_1().into(),
            cnt1_aberr: status1.ab_err_1().into(),
            cnt1_zero: status1.zero_1().into(),
            cnt2_overflow: status2.ovf_2().into(),
            cnt2_aberr: status2.ab_err_2().into(),
            cnt2_zero: status2.zero_2().into(),
            power_status: status0.p_dwn().into(),
            ref_reg_status: status0.r_val().into(),
            upd_reg_status: status0.upd_val().into(),
            ref_cnt_status: status0.ovf_ref().into(),
            ext_err_status: status1.ext_err().into(),
            ext_warn_status: status1.ext_warn().into(),
            comm_status: status1.com_col().into(),
            tp_status: status0.tp_val().into(),
            tpi_status: status1.tps().into(),
            ssi_enabled: status2.en_ssi().into(),
        })
    }

    /// Read the current counter value and return it.
    pub fn read_counter(&mut self) -> Result<CntCount, DeviceError<Spi::Error>> {
        match self.counter_config {
            CntCfg::Cnt1Bit24(_) => {
                let res = self.device.read_cnt_cfg_0().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit24(res.cnt_0()))
            }
            CntCfg::Cnt2Bit24(_, _) => {
                let res = self.device.read_cnt_cfg_1().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit24(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt1Bit48(_) => {
                let res = self.device.read_cnt_cfg_2().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit48(res.cnt_0()))
            }
            CntCfg::Cnt1Bit16(_) => {
                let res = self.device.read_cnt_cfg_3().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit16(res.cnt_0()))
            }
            CntCfg::Cnt1Bit32(_) => {
                let res = self.device.read_cnt_cfg_4().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt1Bit32(res.cnt_0()))
            }
            CntCfg::Cnt2Bit32Bit16(_, _) => {
                let res = self.device.read_cnt_cfg_5().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit32Bit16(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt2Bit16(_, _) => {
                let res = self.device.read_cnt_cfg_6().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt2Bit16(res.cnt_0(), res.cnt_1()))
            }
            CntCfg::Cnt3Bit16(_, _, _) => {
                let res = self.device.read_cnt_cfg_7().read()?;
                self.set_device_status(res.nwarn(), res.nerr());
                Ok(CntCount::Cnt3Bit16(res.cnt_0(), res.cnt_1(), res.cnt_2()))
            }
        }
    }

    /// Reset counters to zero.
    /// You can select which counters should be set to zero using the specific arguments.
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
    pub fn reset_all_counters(&mut self) -> Result<(), DeviceError<Spi::Error>> {
        self.reset_counters(true, true, true)?;
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

    /// Set the counter configuration.
    /// This should be done prior to calling `init()`.
    pub fn set_counter_config(&mut self, config: CntCfg) {
        self.counter_config = config;
    }

    /// Set device status from two bools that were read and passed on to here.
    /// Note taat the inputs are from nerr and nwarn!
    fn set_device_status(&mut self, nwarn: bool, nerr: bool) {
        self.device_status.warning = match nwarn {
            true => WarningStatus::Ok,
            false => WarningStatus::Warning,
        };
        self.device_status.error = match nerr {
            true => ErrorStatus::Ok,
            false => ErrorStatus::Error,
        };
    }
}
