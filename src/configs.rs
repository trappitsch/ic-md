//! Module to hold the configuration and status structs for the device

use core::{convert::From, default::Default, fmt::Debug};

/// Represent the counter values for different configurations of the iC-MD quadrature counter.
///
/// If more than one counter value is present, the counter values are always in the order of
/// Counter 0, Counter 1, and Counter 2.
/// Note: The size of the returned value depends on the configuration of the counter!
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntCount {
    /// Counter return value for configuration counter 0 = 24 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit24(i32),
    /// Counter return value for configuration counter 0 = 24 bit and Counter 1 = 24 bit; 2 counters; TTL only
    Cnt2Bit24(i32, i32),
    /// Counter return value for configuration counter 0 = 48 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit48(i64),
    /// Counter return value for configuration counter 0 = 16 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit16(i16),
    /// Counter return value for configuration counter 0 = 32 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit32(i32),
    /// Counter return value for configuration counter 0 = 32 bit and Counter 1 = 16 bit; 2 counters; TTL only
    Cnt2Bit32Bit16(i16, i32),
    /// Counter return value for configuration counter 0 = 16 bit and Counter 1 = 16 bit; 2 counters; TTL only
    Cnt2Bit16(i16, i16),
    /// Counter return value for configuration counter 0 = 16 bit, Counter 1 = 16 bit, and Counter 2 = 16 bit;
    /// 3 counters; TTL only
    Cnt3Bit16(i16, i16, i16),
}

impl CntCount {
    /// Get the value of the counter zero
    ///
    /// If it exists, this will return `Some(value)`. Otherwise it will return `None`. For counter
    /// zero, this will always exist, as it is always configured.
    pub fn get_cnt0(&self) -> Option<i64> {
        match self {
            CntCount::Cnt1Bit24(val) => Some(*val as i64),
            CntCount::Cnt2Bit24(val, _) => Some(*val as i64),
            CntCount::Cnt1Bit48(val) => Some(*val),
            CntCount::Cnt1Bit16(val) => Some(*val as i64),
            CntCount::Cnt1Bit32(val) => Some(*val as i64),
            CntCount::Cnt2Bit32Bit16(val, _) => Some(*val as i64),
            CntCount::Cnt2Bit16(val, _) => Some(*val as i64),
            CntCount::Cnt3Bit16(val, _, _) => Some(*val as i64),
        }
    }

    /// Get the value of the counter one
    ///
    /// If it exists, this will return `Some(value)`. Otherwise it will return `None`.
    pub fn get_cnt1(&self) -> Option<i64> {
        match self {
            CntCount::Cnt2Bit24(_, val) => Some(*val as i64),
            CntCount::Cnt2Bit32Bit16(_, val) => Some(*val as i64),
            CntCount::Cnt2Bit16(_, val) => Some(*val as i64),
            CntCount::Cnt3Bit16(_, val, _) => Some(*val as i64),
            _ => None,
        }
    }

    /// Get the value of counter two.
    ///
    /// If it exists, this will return `Some(value)`. Otherwise it will return `None`.
    pub fn get_cnt2(&self) -> Option<i64> {
        match self {
            CntCount::Cnt3Bit16(_, _, val) => Some(*val as i64),
            _ => None,
        }
    }
}

/// Enum to specify the direction in which a counter counts
///
/// This enum is used to turn the positive direction of counting around. By default, it is set to
/// CW for positive counting, but can be set to CCW for positive counting.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntDirection {
    /// Clockwise counting direction
    #[default]
    CW,
    /// Counterclockwise counting direction
    CCW,
}

impl From<CntDirection> for u8 {
    fn from(val: CntDirection) -> Self {
        match val {
            CntDirection::CW => 0,
            CntDirection::CCW => 1,
        }
    }
}

/// Enum to specify if the Z signal is normal or inverted
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntZSignal {
    #[default]
    /// Normal Z signal
    Normal,
    /// Inverted Z signal
    Inverted,
}

impl From<CntZSignal> for u8 {
    fn from(val: CntZSignal) -> Self {
        match val {
            CntZSignal::Normal => 0,
            CntZSignal::Inverted => 1,
        }
    }
}

/// Input configuration
///
/// This holds the setup if the inputs are TTL or Differential (RS422 or LVDS).
/// If two or more counters are used, the input setup must be set to TTL.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InputConfig {
    /// Differential RS-422 inputs (default).
    #[default]
    Rs422,
    /// Differential LVDS inputs.
    Lvds,
    /// Single-ended TTL inputs.
    Ttl,
}

impl InputConfig {
    /// Get bit 7 of addr 0x01
    ///
    /// This is the bit that states differential inputs (0, default) or TTL inputs (1).
    pub(crate) fn get_bit7_addr1(&self) -> u8 {
        match self {
            InputConfig::Ttl => 1,
            _ => 0,
        }
    }

    /// Get bit 7 of addr 0x03
    ///
    /// This differs between RS-422 (0, default) and LVDS (1) inputs.
    /// When in TTL mode, this will return 0 (default) even though this bit is ignored.
    pub(crate) fn get_bit7_addr3(&self) -> u8 {
        match self {
            InputConfig::Lvds => 1,
            _ => 0,
        }
    }
}

/// Counter cleared by Z signal.
///
/// This enum inidcates if a given counter is cleared by its Z signal or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntClearedByZ {
    /// Counter is not cleared by Z signal.
    #[default]
    No,
    /// Counter is cleared by Z signal.
    Yes,
}

impl From<&CntClearedByZ> for u8 {
    fn from(val: &CntClearedByZ) -> Self {
        match val {
            CntClearedByZ::No => 0,
            CntClearedByZ::Yes => 1,
        }
    }
}

/// Index signal configuration
///
/// Select when the index signal is active. In the default setup, the index signal is active when A
/// = B = 1 (high).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IndexSignalConfig {
    /// Index signal active when A = B = 1 (high) (default).
    #[default]
    ABHigh,
    /// Index signal active when A = 1 and B = 0.
    AHighBLow,
    /// Index signal active when A = 0 and B = 1.
    ALowBHigh,
    /// Index signal active when A = B = 0 (low).
    ABLow,
}

impl From<IndexSignalConfig> for u8 {
    fn from(val: IndexSignalConfig) -> Self {
        match val {
            IndexSignalConfig::ABHigh => 0b00,
            IndexSignalConfig::AHighBLow => 0b01,
            IndexSignalConfig::ALowBHigh => 0b10,
            IndexSignalConfig::ABLow => 0b11,
        }
    }
}

/// Touch probe pin configuration
///
/// Specify the configuration of the Touch Probe Interface pins, i.e., which edges trigger an
/// event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TouchProbePinConfig {
    /// Both edges (rising and falling) are active (default).
    #[default]
    BothEdges,
    /// Only rising edge is active.
    RisingEdge,
    /// Only falling edge is active.
    FallingEdge,
    /// Pin is disabled (no edge is active).
    Disabled,
}

impl From<TouchProbePinConfig> for u8 {
    fn from(val: TouchProbePinConfig) -> Self {
        match val {
            TouchProbePinConfig::BothEdges => 0b00,
            TouchProbePinConfig::RisingEdge => 0b01,
            TouchProbePinConfig::FallingEdge => 0b10,
            TouchProbePinConfig::Disabled => 0b11,
        }
    }
}

/// Interface priority
///
/// Define the priority of the interfaces, either BiSS or SPI.
/// Note: While the default for the chip is BiSS, this driver (as it is an SPI driver) defaults to
/// SPI priority. Of course, you can overwrite this setting.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterfacePriority {
    /// BiSS interface has priority.
    Biss,
    /// SPI interface has priority (default).
    #[default]
    Spi,
}

impl From<InterfacePriority> for u8 {
    fn from(val: InterfacePriority) -> Self {
        match val {
            InterfacePriority::Biss => 0,
            InterfacePriority::Spi => 1,
        }
    }
}

/// Setup for a specific counter.
///
/// Use this struct to declare the setup of a specific counter.
///
/// The configuration holds the following parameters:
///
/// - Counting direction [`CntDirection`]
/// - Z signal configuration [`CntZSignal`]
/// - Is the counter cleared by the Z signal? [`CntClearedByZ`]
///
/// If three counters are used, no Z signal configuration is possible and thus the
/// `cnt_cleared_by_z` parameter will be ignored as well.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CntSetup {
    count_direction: CntDirection,
    z_signal: CntZSignal,
    cnt_cleared_by_z: CntClearedByZ,
}

impl CntSetup {
    /// Set the counting direction.
    pub fn set_count_direction(&mut self, direction: CntDirection) {
        self.count_direction = direction;
    }

    /// Set the Z signal configuration.
    pub fn set_z_signal(&mut self, z_signal: CntZSignal) {
        self.z_signal = z_signal;
    }

    /// Set if counter 0 cleared by Z signal configuration.
    pub fn set_cnt_cleared_by_z(&mut self, cleared: CntClearedByZ) {
        self.cnt_cleared_by_z = cleared;
    }
}

/// Device configuration
///
/// This sets the overall configuration of the iC-MD device.
/// This configuration includes the input configuration (TTL, RS422, or LVDS), index signal
/// configuration, touch probe pin configuration, and interface priority.
///
/// Example to use a TTL configuration and leaving the rest as default values:
///
/// ```rust
/// use ic_md::{DeviceCfg, };
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceCfg {
    input_config: InputConfig,
    index_signal_config: IndexSignalConfig,
    touch_probe_pin_config: TouchProbePinConfig,
    interface_priority: InterfacePriority,
}

impl DeviceCfg {
    /// Set the input configuration.
    pub fn set_input_config(&mut self, input_config: InputConfig) {
        self.input_config = input_config;
    }

    /// Set the index signal configuration.
    pub fn set_index_signal_config(&mut self, index_signal_config: IndexSignalConfig) {
        self.index_signal_config = index_signal_config;
    }

    /// Set the touch probe pin configuration.
    pub fn set_touch_probe_pin_config(&mut self, touch_probe_pin_config: TouchProbePinConfig) {
        self.touch_probe_pin_config = touch_probe_pin_config;
    }

    /// Set the interface priority.
    pub fn set_interface_priority(&mut self, interface_priority: InterfacePriority) {
        self.interface_priority = interface_priority;
    }

    /// Get the 7 bits to write to address 0x01
    ///
    /// # Arguments
    ///
    /// * `cnt0_cleared_by_z` - Counter 0 cleared by Z signal configuration? This is stored in the
    ///   counter configuration. If not provided, it will default to `0b0`.
    /// * `cnt1_cleared_by_z` - Counter 1 cleared by Z signal configuration? This is stored in the
    ///   counter configuration. If not provided, it will default to `0b0`.
    pub(crate) fn get_addr1(
        &self,
        cnt0_cleared_by_z: Option<&CntClearedByZ>,
        cnt1_cleared_by_z: Option<&CntClearedByZ>,
    ) -> u8 {
        u8::from(self.interface_priority) // bit 0
            | (u8::from(self.touch_probe_pin_config) << 2) // bits 1-2 
            | (u8::from(self.index_signal_config) << 4) // bits 3-4
            | (u8::from(cnt0_cleared_by_z.unwrap_or(&CntClearedByZ::No)) << 5) // bit 5
            | (u8::from(cnt1_cleared_by_z.unwrap_or(&CntClearedByZ::No)) << 6) // bit 6
            | (self.input_config.get_bit7_addr1() << 7) // bit 7
    }

    /// Get the 7 bits to write to address 0x03
    ///
    /// Note: The `NMASK(1:0)` and `MASK(9:8)` are for now just set to zero, as this is not yet
    /// implemented.
    pub(crate) fn get_addr3(&self) -> u8 {
        self.input_config.get_bit7_addr3() << 7 // bit 7
    }
}

/// Counter configuration
///
/// The iC-MD can be configured for 1 up to 3 channels with counter lengths of 16 to 48
/// bits. Each counter can furthermore be specified to count in clockwise or counterclockwise
/// direction. Finally, you can also configure if the Z signal is normal or inverted.
/// For the setup with three counters, the Z signal setup will simply be ignored as there are no
/// connections for Z signals available. See datasheet for more information.
///
/// If two or more counters are used, the input setup must be set to TTL.
///
/// If you enable the `defmt` feature, this enum will contain a `defmt::Format`
/// implementation for logging the current configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntCfg {
    /// Counter 0 = 24 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit24(CntSetup),
    /// Counter 0 = 24 bit and Counter 1 = 24 bit; 2 counters; TTL only
    Cnt2Bit24(CntSetup, CntSetup),
    /// Counter 0 = 48 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit48(CntSetup),
    /// Counter 0 = 16 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit16(CntSetup),
    /// Counter 0 = 32 bit; 1 counter; TTL, RS422, or LVDS
    Cnt1Bit32(CntSetup),
    /// Counter 0 = 32 bit and Counter 1 = 16 bit; 2 counters; TTL only
    Cnt2Bit32Bit16(CntSetup, CntSetup),
    /// Counter 0 = 16 bit and Counter 1 = 16 bit; 2 counters; TTL only
    Cnt2Bit16(CntSetup, CntSetup),
    /// Counter 0 = 16 bit, Counter 1 = 16 bit, and Counter 2 = 16 bit; 3 counters; TTL
    /// only
    Cnt3Bit16(CntSetup, CntSetup, CntSetup),
}

impl CntCfg {
    /// Get references to the `CntClearedByZ` configurations of counter 0 and counter 1 if they
    /// exist.
    pub(crate) fn get_cnt_cleared_by_z(&self) -> (Option<&CntClearedByZ>, Option<&CntClearedByZ>) {
        match self {
            CntCfg::Cnt1Bit24(i) => (Some(&i.cnt_cleared_by_z), None),
            CntCfg::Cnt2Bit24(i, j) => (Some(&i.cnt_cleared_by_z), Some(&j.cnt_cleared_by_z)),
            CntCfg::Cnt1Bit48(i) => (Some(&i.cnt_cleared_by_z), None),
            CntCfg::Cnt1Bit16(i) => (Some(&i.cnt_cleared_by_z), None),
            CntCfg::Cnt1Bit32(i) => (Some(&i.cnt_cleared_by_z), None),
            CntCfg::Cnt2Bit32Bit16(i, j) => (Some(&i.cnt_cleared_by_z), Some(&j.cnt_cleared_by_z)),
            CntCfg::Cnt2Bit16(i, j) => (Some(&i.cnt_cleared_by_z), Some(&j.cnt_cleared_by_z)),
            CntCfg::Cnt3Bit16(_, _, _) => (None, None),
        }
    }
}

impl From<CntCfg> for u8 {
    fn from(val: CntCfg) -> Self {
        match val {
            CntCfg::Cnt1Bit24(i) => {
                // Config is 0b000
                (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg::Cnt2Bit24(i, j) => {
                // Config is 0b001
                0b001
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg::Cnt1Bit48(i) => {
                // Config is 0b010
                0b010 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg::Cnt1Bit16(i) => {
                // Config is 0b011
                0b011 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg::Cnt1Bit32(i) => {
                // Config is 0b100
                0b100 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg::Cnt2Bit32Bit16(i, j) => {
                // Config is 0b101
                0b101
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg::Cnt2Bit16(i, j) => {
                // Config is 0b110
                0b110
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg::Cnt3Bit16(i, j, k) => {
                // Config is 0b111, z signals are ignored as they cannot be connected!
                0b111
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(k.count_direction) << 5)
            }
        }
    }
}

/// Device Status
///
/// This struct describes the status of the device. The variables that indicate if a warning or
/// error has occured. This status is updated whenever the counters are read, as errors and
/// warnings are sent along.
///
/// Note: You are responsible for reading these warnings. Alternatively, you can also query the
/// connected pins `NWARN` and `NERR`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceStatus {
    pub(crate) warning: WarningStatus,
    pub(crate) error: ErrorStatus,
}

impl DeviceStatus {
    /// Return `true` if the device has no errors or warnings, false otherwise.
    pub fn is_ok(&self) -> bool {
        self.warning == WarningStatus::Ok && self.error == ErrorStatus::Ok
    }

    /// Get the current warning status.
    pub fn get_warning(&self) -> WarningStatus {
        self.warning
    }

    /// Get the current error status.
    pub fn get_error(&self) -> ErrorStatus {
        self.error
    }
}

/// Full Device Status
///
/// This struct contains the full status of the device that is returned when reading the status
/// registers. For most registers, reading the status will reset the status bits to `Ok` or the
/// equivalent for the specific status.
///
/// Note: Even if you have only one counter configured, the full device status will still be
/// reported, i.t., other counters (which don't exist in your setup) will also be reported.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FullDeviceStatus {
    /// Overflow of counter 0
    pub cnt0_overflow: OverflowStatus,
    /// Decodification error of AB inputs in counter 0
    pub cnt0_aberr: DecodificationStatus,
    /// Zero status of counter 0
    pub cnt0_zero: ZeroStatus,
    /// Overflow of counter 1
    pub cnt1_overflow: OverflowStatus,
    /// Decodification error of AB inputs in counter 1
    pub cnt1_aberr: DecodificationStatus,
    /// Zero status of counter 1
    pub cnt1_zero: ZeroStatus,
    /// Overflow of counter 2
    pub cnt2_overflow: OverflowStatus,
    /// Decodification error of AB inputs in counter 2
    pub cnt2_aberr: DecodificationStatus,
    /// Zero status of counter 2
    pub cnt2_zero: ZeroStatus,
    /// Power status: Has an undervoltage reset occured?
    pub power_status: UndervoltageStatus,
    /// Reference register status: Is the reference register valid?
    pub ref_reg_status: RegisterStatus,
    /// UPD register status: Is the UPD register valid?
    pub upd_reg_status: RegisterStatus,
    /// Reference counter status.
    pub ref_cnt_status: OverflowStatus,
    /// External error status: Has an external error occured?
    pub ext_err_status: ErrorStatus,
    /// External warning status: Has an external warning occured?
    pub ext_warn_status: WarningStatus,
    /// Communication status: Has a communication collision occured?
    pub comm_status: CommunicationStatus,
    /// Touch probe status: Are the TPx registers updated?
    pub tp_status: TouchProbeStatus,
    /// TPI pin status
    pub tpi_status: PinStatus,
    /// SSI enabled status: Is the SSI interface enabled?
    pub ssi_enabled: InterfaceStatus,
}

/// Actuator status.
///
/// This struct is used to keep track of the status of the actuator pins. Upon first initialization
/// they are both set to `PinStatus::Low`. The actuator pins are ACT0 and ACT1.
#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActuatorStatus {
    /// Status of the ACT0 pin
    pub act0: PinStatus,
    /// Status of the ACT1 pin
    pub act1: PinStatus,
}

/// Warning Status
///
/// Enum that indicates if a warning has occured or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WarningStatus {
    #[default]
    /// No warning has occured.
    Ok,
    /// A warning has occured.
    Warning,
}

impl From<bool> for WarningStatus {
    fn from(val: bool) -> Self {
        match val {
            false => WarningStatus::Ok, // For a real warning, not an NWarn!
            true => WarningStatus::Warning,
        }
    }
}

/// Error Status
///
/// Enum that indicates if an error has occured or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorStatus {
    #[default]
    /// No error has occured.
    Ok,
    /// An error has occured.
    Error,
}

impl From<bool> for ErrorStatus {
    fn from(val: bool) -> Self {
        match val {
            false => ErrorStatus::Ok, // For a real error, not an NErr!
            true => ErrorStatus::Error,
        }
    }
}

/// Decodification Status
///
/// A DecodificationError indicates that either the counting frequency is too high or that
/// two incremental edges are too close together.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodificationStatus {
    #[default]
    /// No decodification error has occured.
    Ok,
    /// A decodification error has occured.
    DecodificationError,
}

impl From<bool> for DecodificationStatus {
    fn from(val: bool) -> Self {
        match val {
            false => DecodificationStatus::Ok,
            true => DecodificationStatus::DecodificationError,
        }
    }
}

/// Overflow Status
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverflowStatus {
    #[default]
    /// No overflow has occured.
    Ok,
    /// An overflow has occured.
    Overflow,
}

impl From<bool> for OverflowStatus {
    fn from(val: bool) -> Self {
        match val {
            false => OverflowStatus::Ok,
            true => OverflowStatus::Overflow,
        }
    }
}

/// Zero Status
///
/// This enum indicates if the counter has reached the zero value or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZeroStatus {
    #[default]
    /// The counter is not at zero.
    NotZero,
    /// The counter is at zero.
    Zero,
}

impl From<bool> for ZeroStatus {
    fn from(val: bool) -> Self {
        match val {
            false => ZeroStatus::NotZero,
            true => ZeroStatus::Zero,
        }
    }
}

/// Power Status
///
/// If VDD falls below the power off supply level, the device is reset and the RAM initialized to
/// the default value. This status bit indicates that this initialization has taken place (and you
/// might want to consider re-initializing the device).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UndervoltageStatus {
    #[default]
    /// The device is running normally and has not been reset due to undervoltage.
    Ok,
    /// The device has been reset due to undervoltage.
    Undervoltage,
}

impl From<bool> for UndervoltageStatus {
    fn from(val: bool) -> Self {
        match val {
            false => UndervoltageStatus::Ok,
            true => UndervoltageStatus::Undervoltage,
        }
    }
}

/// Register Status
///
/// This enum indicates if a register is valid (Ok) or not (Invalid).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterStatus {
    #[default]
    /// The register is valid
    Ok,
    /// The register is not valid
    Invalid,
}

impl From<bool> for RegisterStatus {
    fn from(val: bool) -> Self {
        match val {
            true => RegisterStatus::Ok,
            false => RegisterStatus::Invalid,
        }
    }
}

/// Touch probe Status
///
/// This enum indicates if the TPx registers are not loaded / have not been updated or if new
/// values were loaded into the them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TouchProbeStatus {
    #[default]
    /// The TPx registers have not been updated or are not loaded.
    NotUpdated,
    /// The TPx registers have been updated and contain new values.
    Updated,
}

impl From<bool> for TouchProbeStatus {
    fn from(val: bool) -> Self {
        match val {
            false => TouchProbeStatus::NotUpdated,
            true => TouchProbeStatus::Updated,
        }
    }
}

/// Communication Status
///
/// This enum indicates if the communication with the device has experienced a collision or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommunicationStatus {
    #[default]
    /// No collision has occurred, communication is ok.
    Ok,
    /// A collision has occurred, communication is not ok.
    Collision,
}

impl From<bool> for CommunicationStatus {
    fn from(val: bool) -> Self {
        match val {
            false => CommunicationStatus::Ok,
            true => CommunicationStatus::Collision,
        }
    }
}

/// Interface Status
/// This enum indicates if an interface is enabled or disabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterfaceStatus {
    #[default]
    /// The interface is disabled.
    Disabled,
    /// The interface is enabled.
    Enabled,
}

impl From<bool> for InterfaceStatus {
    fn from(val: bool) -> Self {
        match val {
            false => InterfaceStatus::Disabled,
            true => InterfaceStatus::Enabled,
        }
    }
}

/// Status enum for pins.
///
/// `PinStatus::High` means that the pin is at VDD, `PinStatus::Low` means that the pin is at GND.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinStatus {
    #[default]
    /// Pin is at low level (GND)
    Low,
    /// Pin is at high level (VDD)
    High,
}

impl From<&PinStatus> for bool {
    fn from(val: &PinStatus) -> Self {
        match val {
            PinStatus::High => true,
            PinStatus::Low => false,
        }
    }
}

impl From<bool> for PinStatus {
    fn from(val: bool) -> Self {
        match val {
            true => PinStatus::High,
            false => PinStatus::Low,
        }
    }
}
