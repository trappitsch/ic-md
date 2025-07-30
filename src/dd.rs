//! The iC-MD device driver, created with the `device_driver` crate.
//!
//! Please refer to the iC-MD datasheet to better understand what each command does.

use core::fmt::Debug;

use embedded_hal::spi::{Operation, SpiDevice};

device_driver::create_device! {
    device_name: Device,
    dsl: {
        config {
        type RegisterAddressType = u8;
        }
        /// Counter configuration
        /// The iC-MD can be configured for 1 up to 3 channels with counter lengths of 16 to 48
        /// bits. Here, the counter configuration is selected as a u8 value. The higher-level
        /// driver takes care of converting from a meaningful configuration to the 8-bit value.
        register CounterConfiguration {
            type Access = RW;
            const ADDRESS = 0x00;
            const SIZE_BITS = 8;
            value: uint = 0..8,
        },
        /// Read the 24 bit counter configuration, 24+2 bits to read (4 bytes)
        /// This corresponds to counter configuration `0b000`.
        register ReadCntCfg0 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 32;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 0 value, bits 0-24
            cnt0: int = 8..32,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 24 bit, 2 counters configuration, 48+2 bits to read (7 bytes)
        /// This corresponds to counter configuration `0b001`.
        register ReadCntCfg1 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 56;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 1 value, bits 32-48
            cnt1: int = 32..56,
            /// Counter 0 value, bits 0-24
            cnt0: int = 8..32,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 48 bit counter register, 48+2 bits to read (7 bytes)
        /// This corresponds to counter configuration `0b010`.
        register ReadCntCfg2 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 56;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 0 value, bits 0-48
            cnt0: int = 8..56,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 16 bit counter configuration, 16+2 bits to read (3 bytes)
        /// This corresponds to counter configuration `0b011`.
        register ReadCntCfg3 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 24;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 0 value, bits 0-16
            cnt0: int = 8..24,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 32 bit counter configuration, 32+2 bits to read (5 bytes)
        /// This corresponds to counter configuration `0b100`.
        register ReadCntCfg4 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 40;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 0 value, bits 0-32
            cnt0: int = 8..40,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 32 bit and 16 bit counter configuration, 32+16+2 bits to read (7 bytes)
        /// This corresponds to counter configuration `0b101`.
        register ReadCntCfg5 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 56;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 1 value, bits 16-48
            cnt1: int = 24..56,
            /// Counter 0 value, bits 0-16
            cnt0: int = 8..24,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 16 bit and 16 bit counter configuration, 16+16+2 bits to read (5 bytes)
        /// This corresponds to counter configuration `0b110`.
        register ReadCntCfg6 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 40;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 1 value, bits 16-32
            cnt1: int = 24..40,
            /// Counter 0 value, bits 0-16
            cnt0: int = 8..24,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the 3 x 16 bit counter configuration, 16+16+16+2 bits to read (7 bytes)
        /// This corresponds to counter configuration `0b111`.
        register ReadCntCfg7 {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x08;
            const SIZE_BITS = 64;
            const ALLOW_ADDRESS_OVERLAP = true;

            /// Counter 2 value, bits 32-48
            cnt2: int = 40..56,
            /// Counter 1 value, bits 16-32
            cnt1: int = 24..40,
            /// Counter 0 value, bits 0-16
            cnt0: int = 8..24,
            nerr: bool = 7,
            nwarn: bool = 6,
        },
        /// Read the references registers 24 bits.
        /// TODO: It is unclear if this works, as I assume the address for reading is
        /// auto-incremented as when reading the data. This should be tested once the actual
        /// hardware setup is available with an encoder connected.
        register ReferenceCounter {
            type Access = RO;
            type ByteOrder = BE;
            const ADDRESS = 0x10;
            const SIZE_BITS = 24;
            value: int = 0..24,
        },
        /// Instruction byte (write only)
        /// Allows writing of the instruction bytes. When one of these bits is set to 1, the
        /// corresponding instruction is executed and the bit set back to zero, except in the
        /// case of `Act0` and `Act1`, which remain set to the written value.
        register InstructionByte {
            type Access = WO;
            const ADDRESS = 0x30;
            const SIZE_BITS = 8;

            /// Reset counter 0
            AbRes0: bool = 0,
            /// Reset counter 1
            AbRes1: bool = 1,
            /// Reset counter 2
            AbRes2: bool = 2,
            /// Enable zero codification
            ZCEn: bool = 3,
            /// Load touch probe 2 with touch probe 1 value and touch probe 1 with AB counter value
            TP: bool = 4,
            /// Set actuator pin 0 to VDD if enabled, otherwise to GND
            Act0: bool = 5,
            /// Set actuator pin 1 to VDD if enabled, otherwise to GND
            Act1: bool = 6,
        },
        /// `Status0`: Status of counter 0
        /// Returns the status of counter 0 plus several other status bits. See also `Status1` and
        /// `Status2` for the other counters and more status bits.
        register Status0 {
            type Access = RO;
            const ADDRESS = 0x48;
            const SIZE_BITS = 8;

            /// Touch probe registers TP1/TP2 loaded or new values loaded.
            TpVal: bool = 0,
            /// Overflow of the reference counter. There were too many edges detected between two
            /// index pulses. The value of the UPD and REF registers is not valid.
            OvfRef: bool = 1,
            /// UPD value: Every time that the UPD register is loaded, the status bit UpDval is set
            /// to 1 until the status bit UPD or the register UPD is read out.
            UpdVal: bool = 2,
            /// Status bit that indicates that the reference value was loaded in the REF register
            /// after the "Zero codification" process. After power-on, this bit remains at 0 until
            /// the second different index pulse.
            RVal: bool = 3,
            /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
            /// RAM initialized to the default value. This status bit indicates that this
            /// initialization has taken place.
            PDwn: bool = 4,
            /// Zero of counter 0 reached: The counter has reached the zero value.
            Zero0: bool = 5,
            /// Overflow of counter 0.
            Ovf0: bool = 6,
            /// AB input decodification error for counter 0. It occurs if the counting frequency is
            /// too high or if two incrmeental edges are too close together.
            AbErr0: bool = 7,
        },
        /// `Status1`: Status of counter 1
        /// Returns the status of counter 1 plus several other status bits. See also `Status0` and
        /// `Status2` for the other counters and more status bits.
        register Status1 {
            type Access = RO;
            const ADDRESS = 0x49;
            const SIZE_BITS = 8;

            /// TPS signal: Status of the signal on input pin TPI.
            Tps: bool = 0,
            /// Communication collision took place.
            ComCol: bool = 1,
            /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
            /// outside or set to 0 from inside (an internal masked error has occured).
            ExtWarn: bool = 2,
            /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
            /// outside or set to 0 from inside (an internal masked error has occured).
            ExtErr: bool = 3,
            /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
            /// RAM initialized to the default value. This status bit indicates that this
            /// initialization has taken place.
            PDwn: bool = 4,
            /// Zero of counter 1 reached: The counter has reached the zero value.
            Zero1: bool = 5,
            /// Overflow of counter 1.
            Ovf1: bool = 6,
            /// AB input decodification error for counter 1. It occurs if the counting frequency is
            /// too high or if two incrmeental edges are too close together.
            AbErr1: bool = 7,
        },
        /// `Status2`: Status of counter 2
        /// Returns the status of counter 2 plus several other status bits. See also `Status0` and
        /// `Status1` for the other counters and more status bits.
        register Status2 {
            type Access = RO;
            const ADDRESS = 0x4A;
            const SIZE_BITS = 8;

            /// EnSSI: Status of the SSI pin. If closed, the SSI interface is not enabled and the
            /// status bit is 0. Otherwise, if SSI is enabled (SLI pin is open), the status bit
            /// returns 1.
            EnSsi: bool = 0,
            /// Communication collision took place.
            ComCol: bool = 1,
            /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
            /// outside or set to 0 from inside (an internal masked error has occured).
            ExtWarn: bool = 2,
            /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
            /// outside or set to 0 from inside (an internal masked error has occured).
            ExtErr: bool = 3,
            /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
            /// RAM initialized to the default value. This status bit indicates that this
            /// initialization has taken place.
            PDwn: bool = 4,
            /// Zero of counter 1 reached: The counter has reached the zero value.
            Zero2: bool = 5,
            /// Overflow of counter 1.
            Ovf2: bool = 6,
            /// AB input decodification error for counter 1. It occurs if the counting frequency is
            /// too high or if two incrmeental edges are too close together.
            AbErr2: bool = 7,
        },

    }
}

/// The SPI Device wrapper interface to the driver
#[derive(Debug)]
pub struct DeviceInterface<Spi> {
    /// The SPI device used to communicato with the iC-MD device.
    pub spi: Spi,
}

impl<Spi> DeviceInterface<Spi> {
    /// Construct a new instance of the device.
    ///
    /// Spi mode 0, max 10 MHz according to the datasheet.
    pub const fn new(spi: Spi) -> Self {
        Self { spi }
    }
}

impl<Spi: SpiDevice> device_driver::RegisterInterface for DeviceInterface<Spi> {
    type Error = DeviceError<Spi::Error>;

    type AddressType = u8;

    fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        Ok(SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[address]), Operation::Write(data)],
        )?)
    }

    fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[0x80 | address]), Operation::Read(data)],
        )?;

        Ok(())
    }
}

/// Low level interface error that wraps the SPI error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceError<Spi>(pub Spi);

impl<Spi> From<Spi> for DeviceError<Spi> {
    fn from(value: Spi) -> Self {
        Self(value)
    }
}

impl<Spi> core::ops::Deref for DeviceError<Spi> {
    type Target = Spi;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Spi> core::ops::DerefMut for DeviceError<Spi> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
