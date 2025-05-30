//! This file contains simple tests for the iC-MD device.
//!
//! Each test functon can also be used to study how the iC-MD device can be used in your project.
//! Of course, you will have to provide your own `SPIDevice` interface, however, the rest of these
//! tests should show you how to simple read the device. These tests will be extensively commented
//! for the user to better understand the handling.

use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use ic_md::IcMd;

/// A simple to understand example without any configuration
///
/// Here we set up a counter with the default configuration. We then initialize it and get the
/// results back from the counter. We also make sure that no errors or warnings have occured.
#[test]
fn test_default_icmd_and_counter_read() {
    // SPI transactions - ignore this if you look for the example
    let expectations = [
        Transaction::transaction_start(),
        Transaction::write(0x00),
        Transaction::write(0x02),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write(0x80 | 0x08),
        Transaction::read_vec(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, 0xC0]),
        Transaction::transaction_end(),
    ];

    // Initialize your SPIDevice
    let mut spi_device = Mock::new(&expectations);

    // Get a handle to the counter with the default setup
    let mut icmd = IcMd::new(&mut spi_device);

    // Initialize the counter
    icmd.init().unwrap();

    // Read out the counter
    let counter_value = icmd.read_counter().unwrap();

    // We can use the get counter methods to access the values. This will return an `Option`
    // containing an `i64` value of the count (if the counter is setup, otherwise `None`).
    let cnt_0 = counter_value
        .get_cnt0()
        .expect("Counter 0 should always be set up");

    assert_eq!(cnt_0, 42);

    // Last, let us ensure that there are no errors or warnings in the device status. We can use
    // the `.is_ok()` method on the `DeviceStatus` struct to do this.
    assert!(icmd.get_device_status().is_ok());

    // Check that all our expectations are met - testing only
    spi_device.done();
}
