#![no_main]
#![no_std]

#[macro_use]
extern crate embedded;

use embedded::components::gpio;
use embedded::util::delay;


board!(stm32f429i, {
});

fn main(hw: Hardware) {
    let Hardware { gpio_g, rcc, .. } = hw;

    // power on gpios
    rcc.enable_all_gpio_ports();

    // setup output pin
    let (mut gpio_ctrl, gpio_pins) = gpio_g.split();
    let gpio::GpioPins { pin_13, pin_14, .. } = gpio_pins;

    // FIXME: would a builder-pattern here be nicer?
    let mut p13 = gpio_ctrl.to_write(pin_13,
                                     gpio::Type::PushPull,
                                     gpio::Speed::Low,
                                     gpio::Resistor::NoPull);


    let mut p14 = gpio_ctrl.to_write(pin_14,
                                     gpio::Type::PushPull,
                                     gpio::Speed::Low,
                                     gpio::Resistor::NoPull);



    loop {
        p13.set(false);
        p14.set(true);
        delay(INITIAL_CPU_FREQ);

        p13.set(true);
        p14.set(false);
        delay(INITIAL_CPU_FREQ);
    }
}
