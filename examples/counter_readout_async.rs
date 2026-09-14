//! Read out the iC-MD counts for on real hardware via a
//! [Pico de Gallo](https://crates.io/crates/pico-de-gallo-hal) USB bridge.
//!
//! This examples shows the interactions with the high-level driver implemented with
//! [device-driver](https://crates.io/crates/device-driver) using a blocking interface.
//!
//! The example sets up a new Ic-MD chip and configures it for one counter, 48bit precision with an
//! RS422 interface. It also specifically sets the direciton of the counting such that it can easily
//! be changed later if wanted.
//! The example then starts an endless loop that prints the counter readout regularly to `STDOUT`.
//! Note that Pico de Gallo's default SPI settings are perfectly fine for talking to the iC-MD.
//! The example uses the async interface.
//!
//! ```text
//! cargo run --example counter_readout_async --feature async
//! ```

use std::{thread, time::Duration};

use ic_md::{CntCfg, CntDirection, CntSetup, CntZSignal, IcMdAsync};
use pico_de_gallo_hal::Hal;

#[tokio::main]
async fn main() {
    let hal = Hal::new();
    let spi_dev = hal.spi_device(0).unwrap();

    let mut icmd = IcMdAsync::new(spi_dev);

    // configure the icMD for one counter, 48bit precision
    let counter_setup = CntSetup::new(CntDirection::CW, CntZSignal::Normal);
    let counter_config = CntCfg::Cnt1Bit48(counter_setup);
    icmd.set_counter_config(counter_config);

    // initialize the iC-MD, this writes the config to it.
    icmd.init().await.unwrap();

    loop {
        let counter_value = icmd.read_counter().await.unwrap();
        let cnt_0 = counter_value.get_cnt0().expect("It was just set up.");
        println!("Cnt0 value: {cnt_0}");

        thread::sleep(Duration::from_millis(200));
    }
}
