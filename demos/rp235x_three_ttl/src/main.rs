#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::{info, warn};
use embassy_executor::Spawner;
use embassy_rp::{gpio::{Level, Output}, spi::{self, Spi}};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::{Duration, Timer};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use ic_md::{CntCfg, CntSetup, DeviceCfg, IcMd, InputConfig};


use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // SYSTEM INIT
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let copi= p.PIN_19;
    let cipo = p.PIN_16;
    let clk = p.PIN_18;
    let touch_cs = p.PIN_17;

    // create SPI
    let mut config = spi::Config::default();
    config.frequency = 2_000_000;
    let spi = Spi::new_blocking(p.SPI0, clk, copi, cipo, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
    let mut spi_device = SpiDeviceWithConfig::new(&spi_bus, Output::new(touch_cs, Level::High), config);

    // Create the counter driver, its setup, and initialize it
    let mut icmd = IcMd::new(&mut spi_device);
    let counter_setup = CntCfg::Cnt3Bit16(CntSetup::default(), CntSetup::default(), CntSetup::default());
    let mut device_config = DeviceCfg::default();
    device_config.set_input_config(InputConfig::Ttl);
    icmd.set_counter_config(counter_setup);
    icmd.set_device_config(device_config);
    icmd.init().unwrap();

    // Read all the errors once to clear them.
    let full_status = icmd.get_full_device_status().unwrap();
    info!("Initial full device status: {:?}", full_status);

    loop {
        Timer::after(Duration::from_secs(3)).await;
        let counter_value = icmd.read_counter().unwrap();
        info!("Cnt0: {}", counter_value.get_cnt0().unwrap());

        let device_status = icmd.get_device_status();
        if !device_status.is_ok() {
            warn!("Device status error: {:?}", device_status);
            warn!("Full device status: {:?}", icmd.get_full_device_status().unwrap());
        }
    }
}
