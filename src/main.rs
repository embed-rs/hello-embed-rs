#![no_main]
#![no_std]

#[macro_use]
extern crate embedded;

use embedded::components::gpio;
use embedded::base::volatile::{Volatile, VolatileStruct};
use embedded::util::delay;

// on AHB1, RCC is at 0x4002 3800 - 0x4002 3BFF RCC (DM00071990, pg. 86)
const AHB1_RCC_BASE: u32 = 0x4002_3800;

// DM00031020.pdf, pg 180
const RCC_AHB1ENR: u32 = AHB1_RCC_BASE + 0x30;
// to enable GPIOG, we need to toggle the 6th bit
const RCC_AHB1ENR_GPIOG_EN: u32 = 0b0100_0000;

board!(stm32f429i, {
});

#[repr(C)]
struct SingleItemReg<T> {
    reg: Volatile<T>,
}

impl<T> VolatileStruct for SingleItemReg<T> {}

fn main(hw: Hardware) {
    let mut rcc_ahb1enr =
        unsafe { SingleItemReg::from_ptr(RCC_AHB1ENR as *mut SingleItemReg<u32>) };
    let Hardware { gpio_g, .. } = hw;

    // enable clock on gpio register
    rcc_ahb1enr.reg |= RCC_AHB1ENR_GPIOG_EN;

    let (mut gpio_ctrl, gpio_pins) = gpio_g.split();

    let gpio::GpioPins { pin_13, .. } = gpio_pins;

    let mut p13 = gpio_ctrl.to_write(pin_13,
                                     gpio::Type::PushPull,
                                     gpio::Speed::Low,
                                     gpio::Resistor::NoPull);

    loop {
        p13.set(true);
        delay(INITIAL_CPU_FREQ);

        p13.set(false);
        delay(INITIAL_CPU_FREQ);
    }
}
