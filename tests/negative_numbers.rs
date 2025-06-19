//! This test checks that negative numbers are handled correctly by the IcMd library, no matter how
//! deep the counters are.

use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use ic_md::{CntCfg, CntSetup, IcMd};

/// A simple to understand example without any configuration
///
/// Here we set up a counter with the default configuration. We then initialize it and get the
/// results back from the counter. We also make sure that no errors or warnings have occured.
#[test]
fn test_read_negative_value() {
    // SPI transactions - ignore this if you look for the example
    let expectations = [
        Transaction::transaction_start(),
        Transaction::write(0x00),
        Transaction::write(0x02),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write(0x80 | 0x08),
        Transaction::read_vec(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0]), // -1
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write(0x00),
        Transaction::write(0x01),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write(0x80 | 0x08),
        Transaction::read_vec(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFD, 0xC0]), // -1, -3
        Transaction::transaction_end(),
    ];

    // Initialize your SPIDevice
    let mut spi_device = Mock::new(&expectations);

    // Get a handle to the counter with the default setup
    let mut icmd = IcMd::new(&mut spi_device);

    // Initialize the counter
    icmd.init().unwrap();

    // Now read a counter that has a value of -1. Must come back as 48 bits of `1`.
    let counter_value = icmd.read_counter().unwrap();
    let cnt_0 = counter_value
        .get_cnt0()
        .expect("Counter 0 should always be set up");
    assert_eq!(cnt_0, -1);

    // Set up two 24 bit counters and initialize the counter again
    let counter_setup = CntCfg::Cnt2Bit24(CntSetup::default(), CntSetup::default());
    icmd.set_counter_config(counter_setup);
    icmd.init().unwrap();

    let counter_value = icmd.read_counter().unwrap();
    let cnt_0 = counter_value
        .get_cnt0()
        .expect("Counter 0 should always be set up");
    let cnt_1 = counter_value
        .get_cnt1()
        .expect("Counter 1 should be set up in this configuration");

    assert_eq!(cnt_0, -3); // second one to come back
    assert_eq!(cnt_1, -1); // first one to come back

    // Check that all our expectations are met - testing only
    spi_device.done();
}
