#![no_std]
#![no_main]


use bsp::{entry, hal::pio::PIOExt};
use defmt::*;
use defmt_rtt as _;
use dht_pio::{Dht11, DhtError};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.get_freq().to_Hz());


    let (dht_pio, dht_sm, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut dht = Dht11::new(dht_pio, dht_sm, pins.gpio0.into_function());

    loop {
        info!("Reading from DHT11");
        match dht.read(&mut delay) {
            Ok(data) => {
                debug!(
                    "Temp info:\nHumidity: {}\nTemperature: {}",
                    data.humidity, data.temperature
                );
            },
            Err(error) => {
                match error {
                    DhtError::Timeout => error!("DHT Timeout"),
                    DhtError::ReadError => error!("DHT ReadError"),
                    DhtError::CrcMismatch(one, two) => error!("DHT CrcMismatch: {{ one: {}, two: {} }}", one, two),
                };
            },
        };

        delay.delay_ms(1000);
    }
}
