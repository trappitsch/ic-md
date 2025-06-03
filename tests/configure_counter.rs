//! This file contains a tests that shows how to set up a non-default counter.
//!
//! For your application, you will have to provide your own `SPIDevice` interface.

use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use ic_md::IcMd;

/// Setup a standard counter and query its test status.
#[test]
fn test_default_icmd_and_counter_read() {
    // SPI transactions - ignore this if you look for the example
    let expectations = [
        Transaction::transaction_start(),   // Initialization
        Transaction::write(0x00), 
        Transaction::write(0x4E),
        Transaction::transaction_end(),
        Transaction::transaction_start(),   // Read the counter
        Transaction::write(0x80 | 0x08),
        Transaction::read_vec(vec![0x00, 0x2A, 0x00, 0x0D, 0xC0]),
        Transaction::transaction_end(),
    ];

    // Initialize your SPIDevice
    let mut spi_device = Mock::new(&expectations);

    // Get a handle to the counter with the default setup
    let mut icmd = IcMd::new(&mut spi_device);

    // Specify the non-default counter setup
    // Counter zero: Counter clockwise direction and inverted Z signal
    let cnt0_setup = ic_md::CntSetup::new(ic_md::CntDirection::CCW, ic_md::CntZSignal::Inverted); 
    // Counter one: Clockwise direction and non-inverted Z signal, i.e., default setup
    let cnt1_setup = ic_md::CntSetup::default();

    // We'll set up the two counter setup where each one is 16 bit deep with above defined setups.
    let counter_setup = ic_md::CntCfg::Cnt2Bit16(cnt0_setup, cnt1_setup);

    // Now we set our counter configuration to the iC-MD device and initialize it.
    icmd.set_counter_config(counter_setup);
    icmd.init().unwrap();

    // Read out the counter
    let counter_value = icmd.read_counter().unwrap();  // NWARN is low

    // We can use the get counter methods to access the values. This will return an `Option`
    // containing an `i64` value of the count (if the counter is setup, otherwise `None`).
    let cnt_0 = counter_value
        .get_cnt0()
        .expect("Counter 0 should always be set up");

    assert_eq!(cnt_0, 13);

    // Now we get counter 1, which should be set up in this configuration as well and return 42.
    let cnt_1 = counter_value
        .get_cnt1()
        .expect("Counter 1 should be set up in this configuration");

    assert_eq!(cnt_1, 42);

    // Finally, let us make sure that counter 2 is not set up and returns `None` if queried.
    assert!(counter_value.get_cnt2().is_none());

    // The device status should not be OK, as NWARN is low and thus, we have a warning.
    assert!(icmd.get_device_status().is_ok());

    spi_device.done(); // Ensure all transactions were executed
}
