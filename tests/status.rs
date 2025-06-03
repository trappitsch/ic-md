//! This file contains a tests that shows how to query/handle the device status.
//!
//! For your application, you will have to provide your own `SPIDevice` interface.

use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use ic_md::IcMd;

/// Setup a standard counter and query its test status.
#[test]
fn test_default_icmd_and_counter_read() {
    // SPI transactions - ignore this if you look for the example
    let expectations = [
        Transaction::transaction_start(), // Initialization
        Transaction::write(0x00),
        Transaction::write(0x02),
        Transaction::transaction_end(),
        Transaction::transaction_start(), // Read the counter
        Transaction::write(0x80 | 0x08),
        Transaction::read_vec(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, 0x40]),
        Transaction::transaction_end(),
        Transaction::transaction_start(), // Get the full device status
        Transaction::write(0x48 | 0x80),
        Transaction::read(0x8C), // ABERR0 error
        Transaction::transaction_end(),
        Transaction::transaction_start(), // Get the full device status
        Transaction::write(0x49 | 0x80),
        Transaction::read(0x00), // no error
        Transaction::transaction_end(),
        Transaction::transaction_start(), // Get the full device status
        Transaction::write(0x4A | 0x80),
        Transaction::read(0x00), // no error
        Transaction::transaction_end(),
    ];

    // Initialize your SPIDevice
    let mut spi_device = Mock::new(&expectations);

    // Get a handle to the counter with the default setup
    let mut icmd = IcMd::new(&mut spi_device);

    // Initialize the counter
    icmd.init().unwrap();

    // Read out the counter
    let counter_value = icmd.read_counter().unwrap(); // NWARN is low

    // We can use the get counter methods to access the values. This will return an `Option`
    // containing an `i64` value of the count (if the counter is setup, otherwise `None`).
    let cnt_0 = counter_value
        .get_cnt0()
        .expect("Counter 0 should always be set up");

    assert_eq!(cnt_0, 42);

    // The device status should not be OK, as NWARN is low and thus, we have a warning.
    assert!(!icmd.get_device_status().is_ok());

    // Now we can can get the overall device status (which will also clear the warning).
    let full_status = icmd
        .get_full_device_status()
        .expect("Device status should be available");

    // Make sure we got the correct error back that we defined for this example.
    assert!(full_status.cnt0_aberr == ic_md::DecodificationStatus::DecodificationError);

    // Make sure that all other status flags are Ok.
    // Overflow status for all counters
    assert!(full_status.cnt0_overflow == ic_md::OverflowStatus::Ok);
    assert!(full_status.cnt1_overflow == ic_md::OverflowStatus::Ok);
    assert!(full_status.cnt2_overflow == ic_md::OverflowStatus::Ok);

    // AB decodification status for counters 1 and 2
    assert!(full_status.cnt1_aberr == ic_md::DecodificationStatus::Ok);
    assert!(full_status.cnt2_aberr == ic_md::DecodificationStatus::Ok);

    // None of the counters are zero
    assert!(full_status.cnt0_zero == ic_md::ZeroStatus::NotZero);
    assert!(full_status.cnt1_zero == ic_md::ZeroStatus::NotZero);
    assert!(full_status.cnt2_zero == ic_md::ZeroStatus::NotZero);

    // Power status
    assert!(full_status.power_status == ic_md::UndervoltageStatus::Ok);

    // Reference and UPD registers are okay
    assert!(full_status.ref_reg_status == ic_md::RegisterStatus::Ok);
    assert!(full_status.upd_reg_status == ic_md::RegisterStatus::Ok);

    // Reference counter status not overflowed
    assert!(full_status.ref_cnt_status == ic_md::OverflowStatus::Ok);

    // No exernal errors or warnings
    assert!(full_status.ext_err_status == ic_md::ErrorStatus::Ok);
    assert!(full_status.ext_warn_status == ic_md::WarningStatus::Ok);

    // Communication Status is Ok
    assert!(full_status.comm_status == ic_md::CommunicationStatus::Ok);

    // Touch probe status: Registers not updated
    assert!(full_status.tp_status == ic_md::TouchProbeStatus::NotUpdated);

    // TPI pin status: Low
    assert!(full_status.tpi_status == ic_md::PinStatus::Low);

    // SSI interface status:
    assert!(full_status.ssi_enabled == ic_md::InterfaceStatus::Disabled);

    // Check that all our expectations are met - testing only
    spi_device.done();
}
