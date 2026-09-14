//! The iC-MD device driver, created with the `device_driver` crate.
//!
//! Please refer to the iC-MD datasheet to better understand what each command does.

use core::fmt::Debug;

#[cfg(feature = "blocking")]
pub mod dd_blocking;
#[cfg(feature = "blocking")]
pub use dd_blocking::DeviceInterface;

#[cfg(feature = "async")]
pub mod dd_async;
#[cfg(feature = "async")]
pub use dd_async::DeviceInterfaceAsync;

device_driver::compile! {
    manifest: "icmd.ddsl"
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
