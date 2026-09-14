//! The blocking interface.

use device_driver::RegisterInterfaceBase;
use embedded_hal::spi::{Operation, SpiDevice};

use crate::dd::DeviceError;

/// The SPI Device wrapper interface to the driver
#[derive(Debug)]
pub struct DeviceInterface<Spi> {
    /// The SPI device used to communicate with the iC-MD device.
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

impl<Spi: SpiDevice> RegisterInterfaceBase for DeviceInterface<Spi> {
    type Error = DeviceError<Spi::Error>;
    type AddressType = u8;
}

impl<Spi: SpiDevice> device_driver::RegisterInterface for DeviceInterface<Spi> {
    fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        Ok(SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[address]), Operation::Write(data)],
        )?)
    }

    fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[0x80 | address]), Operation::Read(data)],
        )?;

        Ok(())
    }
}
