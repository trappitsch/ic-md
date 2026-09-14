//! The async interface.

use embedded_hal::spi::Operation;
use embedded_hal_async::spi::SpiDevice;

use crate::dd::DeviceError;

#[derive(Debug)]

pub struct DeviceInterfaceAsync<Spi> {
    /// The SPI device used to communicate with the iC-MD device.
    pub spi: Spi,
}

impl<Spi> DeviceInterfaceAsync<Spi> {
    /// Construct a new instance of the device.
    ///
    /// Spi mode 0, max 10 MHz according to the datasheet.
    pub const fn new(spi: Spi) -> Self {
        Self { spi }
    }
}

impl<Spi: SpiDevice> device_driver::RegisterInterfaceBase for DeviceInterfaceAsync<Spi> {
    type Error = DeviceError<Spi::Error>;
    type AddressType = u8;
}

impl<Spi: SpiDevice> device_driver::AsyncRegisterInterface for DeviceInterfaceAsync<Spi> {
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        Ok(SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[address]), Operation::Write(data)],
        )
        .await?)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _metadata: &device_driver::FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        SpiDevice::transaction(
            &mut self.spi,
            &mut [Operation::Write(&[0x80 | address]), Operation::Read(data)],
        )
        .await?;

        Ok(())
    }
}
