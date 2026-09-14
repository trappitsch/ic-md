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
//! This example requires the "blocking" feature to be activated, which it is by default.
//!
#![cfg_attr(not(feature = "blocking"), doc = "```ignore")]
#![cfg_attr(feature = "blocking", doc = "```")]
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

#![deny(warnings)]
#![cfg_attr(not(test), no_std)]

pub use configs::*;

#[cfg(feature = "blocking")]
pub mod hl_blocking;
#[cfg(feature = "blocking")]
pub use hl_blocking::IcMd;

#[cfg(feature = "async")]
pub mod hl_async;
#[cfg(feature = "async")]
pub use hl_async::IcMdAsync;

pub mod configs;
pub mod dd;
