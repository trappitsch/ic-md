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

/// Enum to specify the direction in which a counter counts
/// This enum is used to turn the positive direction of counting around. By default, it is set to
/// CW for positive counting, but can be set to CCW for positive counting.
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CntSetup {
    count_direction: CntDirection,
    z_signal: CntZSignal,
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
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntCfg2 {
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

impl From<CntCfg2> for u8 {
    // FIXME: Finish this stuff, then test it in the device driver, then replace the
    // conf and set it upt straight. And document, and test, and go. :)
    fn from(val: CntCfg2) -> Self {
        match val {
            CntCfg2::Cnt1Bit24(i) => {
                // Config is 0b000
                (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg2::Cnt2Bit24(i, j) => {
                // Config is 0b001
                0b001
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg2::Cnt1Bit48(i) => {
                // Config is 0b010
                0b010 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg2::Cnt1Bit16(i) => {
                // Config is 0b011
                0b011 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg2::Cnt1Bit32(i) => {
                // Config is 0b100
                0b100 | (u8::from(i.count_direction) << 3) | (u8::from(i.z_signal) << 6)
            }
            CntCfg2::Cnt2Bit32Bit16(i, j) => {
                // Config is 0b101
                0b101
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg2::Cnt2Bit16(i, j) => {
                // Config is 0b110
                0b110
                    | (u8::from(i.count_direction) << 3)
                    | (u8::from(i.z_signal) << 6)
                    | (u8::from(j.count_direction) << 4)
                    | (u8::from(j.z_signal) << 7)
            }
            CntCfg2::Cnt3Bit16(i, j, k) => {
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
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceStatus {
    pub warning: WarningStatus,
    pub error: ErrorStatus,
}

/// Warning Status
/// Enum that indicates if a warning has occured or not.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WarningStatus {
    #[default]
    Ok,
    Warning,
}

impl From<bool> for WarningStatus {
    fn from(val: bool) -> Self {
        match val {
            true => WarningStatus::Ok, // NWarn pin
            false => WarningStatus::Warning,
        }
    }
}

/// Error Status
/// Enum that indicates if an error has occured or not.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorStatus {
    #[default]
    Ok,
    Error,
}

impl From<bool> for ErrorStatus {
    fn from(val: bool) -> Self {
        match val {
            true => ErrorStatus::Ok, // NErr pin
            false => ErrorStatus::Error,
        }
    }
}

/// Status enum for pins.
/// `PinStatus::High` means that the pin is at VDD, `PinStatus::Low` means that the pin is at GND.
#[derive(Debug, Default, Clone, Copy)]
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

/// Actuator status.
/// This struct is used to keep track of the status of the actuator pins. Upon first initialization
/// they are both set to `PinStatus::Low`. The actuator pins are ACT0 and ACT1.
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActuatorStatus {
    pub act0: PinStatus,
    pub act1: PinStatus,
}
