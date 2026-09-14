// This code was generated using device-driver `2.1.0` (),
// a tool distributed under MIT OR Apache-2.0 by Dion Dokter <dev@diondokter.nl>
// 
// For more information about device-driver, visit the website: https://device-driver.com

/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    interface: I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u8,
}
impl<I> Device<I> {
    /// Create a new instance of the device
    pub const fn new(interface: I) -> Self {
        Self { interface, base_address: 0 }
    }
    /// Drop the driver instance and reclaim the interface
    pub fn free(self) -> I {
        self.interface
    }
    /// Counter configuration
    /// The iC-MD can be configured for 1 up to 3 channels with counter lengths of 16 to 48
    /// bits. Here, the counter configuration is selected as a u8 value. The higher-level
    /// driver takes care of converting from a meaningful configuration to the 8-bit value.
    ///
    /// Register operation:
    /// - Address: `0`
    /// - Reset value: `0`
    #[doc(alias = "CounterConfiguration")]
    pub fn counter_configuration(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        CounterConfiguration,
        u8,
        ::device_driver::RW,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 0;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            CounterConfiguration::default,
        )
    }
    /// Read the 24 bit counter configuration, 24+2 bits to read (4 bytes)
    /// This corresponds to counter configuration `0b000`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg0")]
    pub fn read_cnt_cfg_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg0,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg0::default,
        )
    }
    /// Read the 24 bit, 2 counters configuration, 48+2 bits to read (7 bytes)
    /// This corresponds to counter configuration `0b001`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg1")]
    pub fn read_cnt_cfg_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg1,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg1::default,
        )
    }
    /// Read the 48 bit counter register, 48+2 bits to read (7 bytes)
    /// This corresponds to counter configuration `0b010`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg2")]
    pub fn read_cnt_cfg_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg2,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg2::default,
        )
    }
    /// Read the 16 bit counter configuration, 16+2 bits to read (3 bytes)
    /// This corresponds to counter configuration `0b011`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg3")]
    pub fn read_cnt_cfg_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg3,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg3::default,
        )
    }
    /// Read the 32 bit counter configuration, 32+2 bits to read (5 bytes)
    /// This corresponds to counter configuration `0b100`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg4")]
    pub fn read_cnt_cfg_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg4,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg4::default,
        )
    }
    /// Read the 32 bit and 16 bit counter configuration, 32+16+2 bits to read (7 bytes)
    /// This corresponds to counter configuration `0b101`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg5")]
    pub fn read_cnt_cfg_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg5,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg5::default,
        )
    }
    /// Read the 16 bit and 16 bit counter configuration, 16+16+2 bits to read (5 bytes)
    /// This corresponds to counter configuration `0b110`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg6")]
    pub fn read_cnt_cfg_6(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg6,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg6::default,
        )
    }
    /// Read the 3 x 16 bit counter configuration, 16+16+16+2 bits to read (7 bytes)
    /// This corresponds to counter configuration `0b111`.
    ///
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0`
    #[doc(alias = "ReadCntCfg7")]
    pub fn read_cnt_cfg_7(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReadCntCfg7,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReadCntCfg7::default,
        )
    }
    /// Read the references registers 24 bits.
    /// TODO: It is unclear if this works, as I assume the address for reading is
    /// auto-incremented as when reading the data. This should be tested once the actual
    /// hardware setup is available with an encoder connected.
    ///
    /// Register operation:
    /// - Address: `16`
    /// - Reset value: `0`
    #[doc(alias = "ReferenceCounter")]
    pub fn reference_counter(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        ReferenceCounter,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            ReferenceCounter::default,
        )
    }
    /// Instruction byte (write only)
    /// Allows writing of the instruction bytes. When one of these bits is set to 1, the
    /// corresponding instruction is executed and the bit set back to zero, except in the
    /// case of `Act0` and `Act1`, which remain set to the written value.
    ///
    /// Register operation:
    /// - Address: `48`
    /// - Reset value: `0`
    #[doc(alias = "InstructionByte")]
    pub fn instruction_byte(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        InstructionByte,
        u8,
        ::device_driver::WO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 48;
        ::device_driver::RegisterOperation::new(
            self,
            address as u8,
            InstructionByte::default,
        )
    }
    /// `Status0`: Status of counter 0
    /// Returns the status of counter 0 plus several other status bits. See also `Status1` and
    /// `Status2` for the other counters and more status bits.
    ///
    /// Register operation:
    /// - Address: `72`
    /// - Reset value: `0`
    #[doc(alias = "Status0")]
    pub fn status_0(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        Status0,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 72;
        ::device_driver::RegisterOperation::new(self, address as u8, Status0::default)
    }
    /// `Status1`: Status of counter 1
    /// Returns the status of counter 1 plus several other status bits. See also `Status0` and
    /// `Status2` for the other counters and more status bits.
    ///
    /// Register operation:
    /// - Address: `73`
    /// - Reset value: `0`
    #[doc(alias = "Status1")]
    pub fn status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        Status1,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 73;
        ::device_driver::RegisterOperation::new(self, address as u8, Status1::default)
    }
    /// `Status2`: Status of counter 2
    /// Returns the status of counter 2 plus several other status bits. See also `Status0` and
    /// `Status1` for the other counters and more status bits.
    ///
    /// Register operation:
    /// - Address: `74`
    /// - Reset value: `0`
    #[doc(alias = "Status2")]
    pub fn status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<
        '_,
        Self,
        Status2,
        u8,
        ::device_driver::RO,
        (),
    >
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 74;
        ::device_driver::RegisterOperation::new(self, address as u8, Status2::default)
    }
}
impl<I> ::device_driver::Block for Device<I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u8;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        &mut self.interface
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for Status2 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl Status2 {
    /// `bit 0` - Read the `en_ssi` field.
    ///
    /// EnSSI: Status of the SSI pin. If closed, the SSI interface is not enabled and the
    /// status bit is 0. Otherwise, if SSI is enabled (SLI pin is open), the status bit
    /// returns 1.
    #[doc(alias = "EnSsi")]
    #[must_use]
    pub fn en_ssi(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `com_col` field.
    ///
    /// Communication collision took place.
    #[doc(alias = "ComCol")]
    #[must_use]
    pub fn com_col(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `ext_warn` field.
    ///
    /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtWarn")]
    #[must_use]
    pub fn ext_warn(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 3` - Read the `ext_err` field.
    ///
    /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtErr")]
    #[must_use]
    pub fn ext_err(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 4` - Read the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    #[must_use]
    pub fn p_dwn(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `zero_2` field.
    ///
    /// Zero of counter 1 reached: The counter has reached the zero value.
    #[doc(alias = "Zero2")]
    #[must_use]
    pub fn zero_2(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `ovf_2` field.
    ///
    /// Overflow of counter 1.
    #[doc(alias = "Ovf2")]
    #[must_use]
    pub fn ovf_2(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `ab_err_2` field.
    ///
    /// AB input decodification error for counter 1. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr2")]
    #[must_use]
    pub fn ab_err_2(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `en_ssi` field.
    ///
    /// EnSSI: Status of the SSI pin. If closed, the SSI interface is not enabled and the
    /// status bit is 0. Otherwise, if SSI is enabled (SLI pin is open), the status bit
    /// returns 1.
    #[doc(alias = "EnSsi")]
    pub fn set_en_ssi(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `com_col` field.
    ///
    /// Communication collision took place.
    #[doc(alias = "ComCol")]
    pub fn set_com_col(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `ext_warn` field.
    ///
    /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtWarn")]
    pub fn set_ext_warn(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `ext_err` field.
    ///
    /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtErr")]
    pub fn set_ext_err(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    pub fn set_p_dwn(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `zero_2` field.
    ///
    /// Zero of counter 1 reached: The counter has reached the zero value.
    #[doc(alias = "Zero2")]
    pub fn set_zero_2(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `ovf_2` field.
    ///
    /// Overflow of counter 1.
    #[doc(alias = "Ovf2")]
    pub fn set_ovf_2(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `ab_err_2` field.
    ///
    /// AB input decodification error for counter 1. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr2")]
    pub fn set_ab_err_2(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Status2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for Status2 {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<Status2> for [u8; 1] {
    fn from(val: Status2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Status2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Status2");
        d.field("en_ssi", &self.en_ssi());
        d.field("com_col", &self.com_col());
        d.field("ext_warn", &self.ext_warn());
        d.field("ext_err", &self.ext_err());
        d.field("p_dwn", &self.p_dwn());
        d.field("zero_2", &self.zero_2());
        d.field("ovf_2", &self.ovf_2());
        d.field("ab_err_2", &self.ab_err_2());
        d.finish()
    }
}
impl core::ops::BitAnd for Status2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Status2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Status2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Status2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Status2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Status2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Status2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for Status1 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl Status1 {
    /// `bit 0` - Read the `tps` field.
    ///
    /// TPS signal: Status of the signal on input pin TPI.
    #[doc(alias = "Tps")]
    #[must_use]
    pub fn tps(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `com_col` field.
    ///
    /// Communication collision took place.
    #[doc(alias = "ComCol")]
    #[must_use]
    pub fn com_col(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `ext_warn` field.
    ///
    /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtWarn")]
    #[must_use]
    pub fn ext_warn(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 3` - Read the `ext_err` field.
    ///
    /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtErr")]
    #[must_use]
    pub fn ext_err(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 4` - Read the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    #[must_use]
    pub fn p_dwn(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `zero_1` field.
    ///
    /// Zero of counter 1 reached: The counter has reached the zero value.
    #[doc(alias = "Zero1")]
    #[must_use]
    pub fn zero_1(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `ovf_1` field.
    ///
    /// Overflow of counter 1.
    #[doc(alias = "Ovf1")]
    #[must_use]
    pub fn ovf_1(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `ab_err_1` field.
    ///
    /// AB input decodification error for counter 1. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr1")]
    #[must_use]
    pub fn ab_err_1(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `tps` field.
    ///
    /// TPS signal: Status of the signal on input pin TPI.
    #[doc(alias = "Tps")]
    pub fn set_tps(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `com_col` field.
    ///
    /// Communication collision took place.
    #[doc(alias = "ComCol")]
    pub fn set_com_col(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `ext_warn` field.
    ///
    /// ExtWarn: Status bit that indicates if the `NWARN` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtWarn")]
    pub fn set_ext_warn(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `ext_err` field.
    ///
    /// ExtErr: Status bit that indicates if the `NERR` pin was either pulled-down from
    /// outside or set to 0 from inside (an internal masked error has occured).
    #[doc(alias = "ExtErr")]
    pub fn set_ext_err(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    pub fn set_p_dwn(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `zero_1` field.
    ///
    /// Zero of counter 1 reached: The counter has reached the zero value.
    #[doc(alias = "Zero1")]
    pub fn set_zero_1(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `ovf_1` field.
    ///
    /// Overflow of counter 1.
    #[doc(alias = "Ovf1")]
    pub fn set_ovf_1(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `ab_err_1` field.
    ///
    /// AB input decodification error for counter 1. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr1")]
    pub fn set_ab_err_1(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Status1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for Status1 {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<Status1> for [u8; 1] {
    fn from(val: Status1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Status1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Status1");
        d.field("tps", &self.tps());
        d.field("com_col", &self.com_col());
        d.field("ext_warn", &self.ext_warn());
        d.field("ext_err", &self.ext_err());
        d.field("p_dwn", &self.p_dwn());
        d.field("zero_1", &self.zero_1());
        d.field("ovf_1", &self.ovf_1());
        d.field("ab_err_1", &self.ab_err_1());
        d.finish()
    }
}
impl core::ops::BitAnd for Status1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Status1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Status1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Status1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Status1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Status1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Status1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Status0 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for Status0 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl Status0 {
    /// `bit 0` - Read the `tp_val` field.
    ///
    /// Touch probe registers TP1/TP2 loaded or new values loaded.
    #[doc(alias = "TpVal")]
    #[must_use]
    pub fn tp_val(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `ovf_ref` field.
    ///
    /// Overflow of the reference counter. There were too many edges detected between two
    /// index pulses. The value of the UPD and REF registers is not valid.
    #[doc(alias = "OvfRef")]
    #[must_use]
    pub fn ovf_ref(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `upd_val` field.
    ///
    /// UPD value: Every time that the UPD register is loaded, the status bit UpDval is set
    /// to 1 until the status bit UPD or the register UPD is read out.
    #[doc(alias = "UpdVal")]
    #[must_use]
    pub fn upd_val(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 3` - Read the `r_val` field.
    ///
    /// Status bit that indicates that the reference value was loaded in the REF register
    /// after the "Zero codification" process. After power-on, this bit remains at 0 until
    /// the second different index pulse.
    #[doc(alias = "RVal")]
    #[must_use]
    pub fn r_val(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 4` - Read the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    #[must_use]
    pub fn p_dwn(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `zero_0` field.
    ///
    /// Zero of counter 0 reached: The counter has reached the zero value.
    #[doc(alias = "Zero0")]
    #[must_use]
    pub fn zero_0(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `ovf_0` field.
    ///
    /// Overflow of counter 0.
    #[doc(alias = "Ovf0")]
    #[must_use]
    pub fn ovf_0(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 7` - Read the `ab_err_0` field.
    ///
    /// AB input decodification error for counter 0. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr0")]
    #[must_use]
    pub fn ab_err_0(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `tp_val` field.
    ///
    /// Touch probe registers TP1/TP2 loaded or new values loaded.
    #[doc(alias = "TpVal")]
    pub fn set_tp_val(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `ovf_ref` field.
    ///
    /// Overflow of the reference counter. There were too many edges detected between two
    /// index pulses. The value of the UPD and REF registers is not valid.
    #[doc(alias = "OvfRef")]
    pub fn set_ovf_ref(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `upd_val` field.
    ///
    /// UPD value: Every time that the UPD register is loaded, the status bit UpDval is set
    /// to 1 until the status bit UPD or the register UPD is read out.
    #[doc(alias = "UpdVal")]
    pub fn set_upd_val(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `r_val` field.
    ///
    /// Status bit that indicates that the reference value was loaded in the REF register
    /// after the "Zero codification" process. After power-on, this bit remains at 0 until
    /// the second different index pulse.
    #[doc(alias = "RVal")]
    pub fn set_r_val(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `p_dwn` field.
    ///
    /// Power down: If VDD reaches the power off supply level, the iC-MD is reset and the
    /// RAM initialized to the default value. This status bit indicates that this
    /// initialization has taken place.
    #[doc(alias = "PDwn")]
    pub fn set_p_dwn(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `zero_0` field.
    ///
    /// Zero of counter 0 reached: The counter has reached the zero value.
    #[doc(alias = "Zero0")]
    pub fn set_zero_0(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `ovf_0` field.
    ///
    /// Overflow of counter 0.
    #[doc(alias = "Ovf0")]
    pub fn set_ovf_0(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `ab_err_0` field.
    ///
    /// AB input decodification error for counter 0. It occurs if the counting frequency is
    /// too high or if two incrmeental edges are too close together.
    #[doc(alias = "AbErr0")]
    pub fn set_ab_err_0(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for Status0 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for Status0 {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<Status0> for [u8; 1] {
    fn from(val: Status0) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Status0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Status0");
        d.field("tp_val", &self.tp_val());
        d.field("ovf_ref", &self.ovf_ref());
        d.field("upd_val", &self.upd_val());
        d.field("r_val", &self.r_val());
        d.field("p_dwn", &self.p_dwn());
        d.field("zero_0", &self.zero_0());
        d.field("ovf_0", &self.ovf_0());
        d.field("ab_err_0", &self.ab_err_0());
        d.finish()
    }
}
impl core::ops::BitAnd for Status0 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Status0 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Status0 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Status0 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Status0 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Status0 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Status0 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct InstructionByte {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for InstructionByte {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl InstructionByte {
    /// `bit 0` - Read the `ab_res_0` field.
    ///
    /// Reset counter 0
    #[doc(alias = "AbRes0")]
    #[must_use]
    pub fn ab_res_0(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 1` - Read the `ab_res_1` field.
    ///
    /// Reset counter 1
    #[doc(alias = "AbRes1")]
    #[must_use]
    pub fn ab_res_1(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 2` - Read the `ab_res_2` field.
    ///
    /// Reset counter 2
    #[doc(alias = "AbRes2")]
    #[must_use]
    pub fn ab_res_2(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 3` - Read the `zc_en` field.
    ///
    /// Enable zero codification
    #[doc(alias = "ZCEn")]
    #[must_use]
    pub fn zc_en(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 4` - Read the `tp` field.
    ///
    /// Load touch probe 2 with touch probe 1 value and touch probe 1 with AB counter value
    #[doc(alias = "TP")]
    #[must_use]
    pub fn tp(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 5` - Read the `act_0` field.
    ///
    /// Set actuator pin 0 to VDD if enabled, otherwise to GND
    #[doc(alias = "Act0")]
    #[must_use]
    pub fn act_0(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `act_1` field.
    ///
    /// Set actuator pin 1 to VDD if enabled, otherwise to GND
    #[doc(alias = "Act1")]
    #[must_use]
    pub fn act_1(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 0` - Set the `ab_res_0` field.
    ///
    /// Reset counter 0
    #[doc(alias = "AbRes0")]
    pub fn set_ab_res_0(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 1` - Set the `ab_res_1` field.
    ///
    /// Reset counter 1
    #[doc(alias = "AbRes1")]
    pub fn set_ab_res_1(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 2` - Set the `ab_res_2` field.
    ///
    /// Reset counter 2
    #[doc(alias = "AbRes2")]
    pub fn set_ab_res_2(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 3` - Set the `zc_en` field.
    ///
    /// Enable zero codification
    #[doc(alias = "ZCEn")]
    pub fn set_zc_en(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 4` - Set the `tp` field.
    ///
    /// Load touch probe 2 with touch probe 1 value and touch probe 1 with AB counter value
    #[doc(alias = "TP")]
    pub fn set_tp(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 5` - Set the `act_0` field.
    ///
    /// Set actuator pin 0 to VDD if enabled, otherwise to GND
    #[doc(alias = "Act0")]
    pub fn set_act_0(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `act_1` field.
    ///
    /// Set actuator pin 1 to VDD if enabled, otherwise to GND
    #[doc(alias = "Act1")]
    pub fn set_act_1(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for InstructionByte {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for InstructionByte {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<InstructionByte> for [u8; 1] {
    fn from(val: InstructionByte) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for InstructionByte {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("InstructionByte");
        d.field("ab_res_0", &self.ab_res_0());
        d.field("ab_res_1", &self.ab_res_1());
        d.field("ab_res_2", &self.ab_res_2());
        d.field("zc_en", &self.zc_en());
        d.field("tp", &self.tp());
        d.field("act_0", &self.act_0());
        d.field("act_1", &self.act_1());
        d.finish()
    }
}
impl core::ops::BitAnd for InstructionByte {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for InstructionByte {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for InstructionByte {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for InstructionByte {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for InstructionByte {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for InstructionByte {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for InstructionByte {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReferenceCounter {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 3],
}
unsafe impl ::device_driver::Fieldset for ReferenceCounter {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 3] };
}
impl ReferenceCounter {
    /// `23:0` - Read the `value` field.
    ///
    #[must_use]
    pub fn value(&self) -> i32 {
        let start = 0;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `23:0` - Set the `value` field.
    ///
    pub fn set_value(&mut self, value: i32) {
        let start = 0;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReferenceCounter {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 3]> for ReferenceCounter {
    fn from(bits: [u8; 3]) -> Self {
        Self { bits }
    }
}
impl From<ReferenceCounter> for [u8; 3] {
    fn from(val: ReferenceCounter) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReferenceCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReferenceCounter");
        d.field("value", &self.value());
        d.finish()
    }
}
impl core::ops::BitAnd for ReferenceCounter {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReferenceCounter {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReferenceCounter {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReferenceCounter {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReferenceCounter {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReferenceCounter {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReferenceCounter {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg7 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg7 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl ReadCntCfg7 {
    /// `55:40` - Read the `cnt_2` field.
    ///
    /// Counter 2 value, bits 32-48
    #[doc(alias = "cnt2")]
    #[must_use]
    pub fn cnt_2(&self) -> i16 {
        let start = 40;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `39:24` - Read the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-32
    #[doc(alias = "cnt1")]
    #[must_use]
    pub fn cnt_1(&self) -> i16 {
        let start = 24;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `23:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `55:40` - Set the `cnt_2` field.
    ///
    /// Counter 2 value, bits 32-48
    #[doc(alias = "cnt2")]
    pub fn set_cnt_2(&mut self, value: i16) {
        let start = 40;
        let end = 55;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `39:24` - Set the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-32
    #[doc(alias = "cnt1")]
    pub fn set_cnt_1(&mut self, value: i16) {
        let start = 24;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg7 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for ReadCntCfg7 {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg7> for [u8; 8] {
    fn from(val: ReadCntCfg7) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg7");
        d.field("cnt_2", &self.cnt_2());
        d.field("cnt_1", &self.cnt_1());
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg7 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg7 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg7 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg7 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg7 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg7 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg7 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg6 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 5],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg6 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 5] };
}
impl ReadCntCfg6 {
    /// `39:24` - Read the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-32
    #[doc(alias = "cnt1")]
    #[must_use]
    pub fn cnt_1(&self) -> i16 {
        let start = 24;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `23:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `39:24` - Set the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-32
    #[doc(alias = "cnt1")]
    pub fn set_cnt_1(&mut self, value: i16) {
        let start = 24;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg6 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 5]> for ReadCntCfg6 {
    fn from(bits: [u8; 5]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg6> for [u8; 5] {
    fn from(val: ReadCntCfg6) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg6");
        d.field("cnt_1", &self.cnt_1());
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg6 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg6 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg6 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg6 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg6 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg6 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg6 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg5 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 7],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg5 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 7] };
}
impl ReadCntCfg5 {
    /// `55:24` - Read the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-48
    #[doc(alias = "cnt1")]
    #[must_use]
    pub fn cnt_1(&self) -> i32 {
        let start = 24;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `23:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `55:24` - Set the `cnt_1` field.
    ///
    /// Counter 1 value, bits 16-48
    #[doc(alias = "cnt1")]
    pub fn set_cnt_1(&mut self, value: i32) {
        let start = 24;
        let end = 55;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `23:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg5 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 7]> for ReadCntCfg5 {
    fn from(bits: [u8; 7]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg5> for [u8; 7] {
    fn from(val: ReadCntCfg5) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg5");
        d.field("cnt_1", &self.cnt_1());
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg5 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg5 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg5 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg5 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg5 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg5 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg5 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 5],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg4 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 5] };
}
impl ReadCntCfg4 {
    /// `39:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-32
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i32 {
        let start = 8;
        let end = 39;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `39:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-32
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i32) {
        let start = 8;
        let end = 39;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 5]> for ReadCntCfg4 {
    fn from(bits: [u8; 5]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg4> for [u8; 5] {
    fn from(val: ReadCntCfg4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg4");
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 3],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg3 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 3] };
}
impl ReadCntCfg3 {
    /// `23:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i16 {
        let start = 8;
        let end = 23;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i16,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `23:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-16
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i16) {
        let start = 8;
        let end = 23;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i16,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 3]> for ReadCntCfg3 {
    fn from(bits: [u8; 3]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg3> for [u8; 3] {
    fn from(val: ReadCntCfg3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg3");
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 7],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg2 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 7] };
}
impl ReadCntCfg2 {
    /// `55:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-48
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i64 {
        let start = 8;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i64,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `55:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-48
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i64) {
        let start = 8;
        let end = 55;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i64,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 7]> for ReadCntCfg2 {
    fn from(bits: [u8; 7]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg2> for [u8; 7] {
    fn from(val: ReadCntCfg2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg2");
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 7],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg1 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 7] };
}
impl ReadCntCfg1 {
    /// `55:32` - Read the `cnt_1` field.
    ///
    /// Counter 1 value, bits 32-48
    #[doc(alias = "cnt1")]
    #[must_use]
    pub fn cnt_1(&self) -> i32 {
        let start = 32;
        let end = 55;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `31:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-24
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i32 {
        let start = 8;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `55:32` - Set the `cnt_1` field.
    ///
    /// Counter 1 value, bits 32-48
    #[doc(alias = "cnt1")]
    pub fn set_cnt_1(&mut self, value: i32) {
        let start = 32;
        let end = 55;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `31:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-24
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i32) {
        let start = 8;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 7]> for ReadCntCfg1 {
    fn from(bits: [u8; 7]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg1> for [u8; 7] {
    fn from(val: ReadCntCfg1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg1");
        d.field("cnt_1", &self.cnt_1());
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ReadCntCfg0 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for ReadCntCfg0 {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::BE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl ReadCntCfg0 {
    /// `31:8` - Read the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-24
    #[doc(alias = "cnt0")]
    #[must_use]
    pub fn cnt_0(&self) -> i32 {
        let start = 8;
        let end = 31;
        let raw = unsafe {
            ::device_driver::ops::load::<
                i32,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `bit 7` - Read the `nerr` field.
    ///
    #[must_use]
    pub fn nerr(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `bit 6` - Read the `nwarn` field.
    ///
    #[must_use]
    pub fn nwarn(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::BE,
            >(&self.bits, start, end)
        };
        raw > 0
    }
    /// `31:8` - Set the `cnt_0` field.
    ///
    /// Counter 0 value, bits 0-24
    #[doc(alias = "cnt0")]
    pub fn set_cnt_0(&mut self, value: i32) {
        let start = 8;
        let end = 31;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                i32,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 7` - Set the `nerr` field.
    ///
    pub fn set_nerr(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
    /// `bit 6` - Set the `nwarn` field.
    ///
    pub fn set_nwarn(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::BE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for ReadCntCfg0 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for ReadCntCfg0 {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<ReadCntCfg0> for [u8; 4] {
    fn from(val: ReadCntCfg0) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ReadCntCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ReadCntCfg0");
        d.field("cnt_0", &self.cnt_0());
        d.field("nerr", &self.nerr());
        d.field("nwarn", &self.nwarn());
        d.finish()
    }
}
impl core::ops::BitAnd for ReadCntCfg0 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ReadCntCfg0 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ReadCntCfg0 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ReadCntCfg0 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ReadCntCfg0 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ReadCntCfg0 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ReadCntCfg0 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CounterConfiguration {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for CounterConfiguration {
    const METADATA: ::device_driver::FieldsetMetadata = ::device_driver::FieldsetMetadata::new()
        .with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl CounterConfiguration {
    /// `7:0` - Read the `value` field.
    ///
    #[must_use]
    pub fn value(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe {
            ::device_driver::ops::load::<
                u8,
                ::device_driver::ops::LE,
            >(&self.bits, start, end)
        };
        raw
    }
    /// `7:0` - Set the `value` field.
    ///
    pub fn set_value(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe {
            ::device_driver::ops::store::<
                u8,
                ::device_driver::ops::LE,
            >(raw, start, end, &mut self.bits)
        };
    }
}
impl Default for CounterConfiguration {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for CounterConfiguration {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<CounterConfiguration> for [u8; 1] {
    fn from(val: CounterConfiguration) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CounterConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CounterConfiguration");
        d.field("value", &self.value());
        d.finish()
    }
}
impl core::ops::BitAnd for CounterConfiguration {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CounterConfiguration {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CounterConfiguration {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CounterConfiguration {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CounterConfiguration {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CounterConfiguration {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CounterConfiguration {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
