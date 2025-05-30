// Hold all the configuration and status structs for the device

use core::{convert::From, default::Default, fmt::Debug};

/// Represent the counter values for different configurations of the iC-MD quadrature counter.
/// If more than one counter value is present, the counter values are always in the order of
/// Counter 0, Counter 1, and Counter 2.
/// Note: The size of the returned value depends on the configuration of the counter!
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntCount {
    Cnt1Bit24(i32),
    Cnt2Bit24(i32, i32),
    Cnt1Bit48(i64),
    Cnt1Bit16(i16),
    Cnt1Bit32(i32),
    Cnt2Bit32Bit16(i16, i32),
    Cnt2Bit16(i16, i16),
    Cnt3Bit16(i16, i16, i16),
}

impl CntCount {
    /// Get the value of the counter zero
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
    /// If it exists, this will return `Some(value)`. Otherwise it will return `None`.
    pub fn get_cnt2(&self) -> Option<i64> {
        match self {
            CntCount::Cnt3Bit16(_, _, val) => Some(*val as i64),
            _ => None,
        }
    }
}

/// Enum to specify the direction in which a counter counts
/// This enum is used to turn the positive direction of counting around. By default, it is set to
/// CW for positive counting, but can be set to CCW for positive counting.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntDirection {
    #[default]
    CW,
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
    Normal,
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

/// Setup for a specific counter.
/// Use this struct to declare the setup of a specific counter.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CntSetup {
    count_direction: CntDirection,
    z_signal: CntZSignal,
}

impl CntSetup {
    /// Create a new counter setup with the given direction and Z signal.
    pub fn new(count_direction: CntDirection, z_signal: CntZSignal) -> Self {
        Self {
            count_direction,
            z_signal,
        }
    }
}

/// Counter configuration
/// The iC-MD can be configured for 1 up to 3 channels with counter lengths of 16 to 48
/// bits. Each counter can furthermore be specified to count in clockwise or counterclockwise
/// direction. Finally, you can also configure if the Z signal is normal or inverted.
/// For the setup with three counters, the Z signal setup will simply be ignored as there are no
/// connections for Z signals available. See datasheet for more information.
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
    // Touch probe status: Are the TPx registers updated?
    pub tp_status: TouchProbeStatus,
    /// TPI pin status
    pub tpi_status: PinStatus,
    /// SSI enabled status: Is the SSI interface enabled?
    pub ssi_enabled: InterfaceStatus,
}

/// Actuator status.
/// This struct is used to keep track of the status of the actuator pins. Upon first initialization
/// they are both set to `PinStatus::Low`. The actuator pins are ACT0 and ACT1.
#[derive(Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActuatorStatus {
    pub act0: PinStatus,
    pub act1: PinStatus,
}

/// Warning Status
/// Enum that indicates if a warning has occured or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WarningStatus {
    #[default]
    Ok,
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
/// Enum that indicates if an error has occured or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorStatus {
    #[default]
    Ok,
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
/// A DecodificationError indicates that either the counting frequency is too high or that
/// two incremental edges are too close together.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodificationStatus {
    #[default]
    Ok,
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
    Ok,
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
/// This enum indicates if the counter has reached the zero value or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZeroStatus {
    #[default]
    NotZero,
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
/// This enum indicates if a register is valid (Ok) or not (Invalid).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterStatus {
    #[default]
    Ok,
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
/// This enum indicates if the TPx registers are not loaded / have not been updated or if new
/// values were loaded into the them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TouchProbeStatus {
    #[default]
    NotUpdated,
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

/// Communiucatoion Status
/// This enum indicates if the communication with the device has experienced a collision or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommunicationStatus {
    #[default]
    Ok,
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
    Disabled,
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
/// `PinStatus::High` means that the pin is at VDD, `PinStatus::Low` means that the pin is at GND.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinStatus {
    #[default]
    Low,
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
