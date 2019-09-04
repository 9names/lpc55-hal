#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m_semihosting::dbg;

use lpc55s6x_hal as hal;
use hal::{
    prelude::*,
    gpio::{self, Level},
    iocon,
};

#[rtfm::app(device = crate::hal::raw, peripherals = true)]
const APP: () = {
    struct Resources {
        led: iocon::Pin<iocon::PIO1_6, iocon::pin_state::Gpio<gpio::direction::Output>>,
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        dbg!("init");
        let _cp = c.core;
        let dp = c.device;

        // setup red LED
        let mut syscon = hal::syscon::SYSCON::new(dp.SYSCON).split();
        let gpio = hal::gpio::GPIO::new(dp.GPIO).enable(&mut syscon.handle);
        let iocon = hal::iocon::IOCON::new(dp.IOCON).split();

        let red_led = iocon.pins.pio1_6
            .into_gpio_pin(&gpio)
            .into_output(Level::High);

        init::LateResources { led: red_led }
    }

    #[idle(resources = [led])]
    fn idle(c: idle::Context) -> ! {
        let led = c.resources.led;
        loop {
            dbg!("low");
            led.set_low().unwrap();

            dbg!("high");
            led.set_high().unwrap();
        }
    }
};