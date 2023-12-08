#![no_std]
#![no_main]

use bsp::{entry, hal::clocks::ClockSource};
use defmt::*;
use defmt_rtt as _;
use dht_sensor::DhtReading;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};

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

    let mut dht11_pin = bsp::hal::gpio::InOutPin::new(pins.gpio28);
    match dht11_pin.set_high() {
        Ok(value) => info!("Set GPIO2 HIGH: {}", value),
        Err(err) => error!("Failed to set GPIO2 HIGH: {}", err),
    };

    loop {
        info!("Reading from DHT11");
        match dht_sensor::dht11::Reading::read(&mut delay, &mut dht11_pin) {
            Ok(data) => {
                debug!(
                    "Temp info:\nHumidity: {}\nTemperature: {}",
                    data.relative_humidity, data.temperature
                );
            }
            Err(error) => {
                match error {
                    dht_sensor::DhtError::Timeout => {
                        error!("Timeout");
                    }
                    dht_sensor::DhtError::PinError(_e) => error!("DHT PinError"),
                    dht_sensor::DhtError::ChecksumMismatch => error!("DHT ChecksumMismatch"),
                }
            }
        };

        delay.delay_ms(1000);
    }
}

// End of file
