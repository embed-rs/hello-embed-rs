#![no_std]

#[macro_use]
extern crate embedded;

use embedded::components::gpio;
use embedded::base::volatile::{Volatile, VolatileStruct};
use embedded::components::gpio::GpioController;
use embedded::util::delay;

// on AHB1, GPIOG is at 0x4002 1800 - 0x4002 1BFF (DM00071990, pg. 86)
const AHB1_GPIOG_BASE: u32 = 0x4002_1800;

// on AHB1, RCC is at 0x4002 3800 - 0x4002 3BFF RCC (DM00071990, pg. 86)
const AHB1_RCC_BASE: u32 = 0x4002_3800;

// base address of systick timer (guide pg. 313ff)
const SYSTICK_BASE: u32 = 0xE000E010;

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

fn main() {

    let mut rcc_ahb1enr =
        unsafe { SingleItemReg::from_ptr(RCC_AHB1ENR as *mut SingleItemReg<u32>) };
    let mut gpio_g = unsafe {
        gpio::GpioBank::<gpio::PortG>::from_ptr(AHB1_GPIOG_BASE as *mut gpio::GpioBank<gpio::PortG>)
    };

    // enable clock on gpio register
    rcc_ahb1enr.reg |= RCC_AHB1ENR_GPIOG_EN;

    let (mut gpio_ctrl, gpio_pins) = gpio_g.split();

    let gpio::GpioPins { pin_13, pin_14, .. } = gpio_pins;

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

    // setup gpio pins 13 and 14
    // gpio_setup!(gpio_g.split(), {
    //     pin_13 = output(pin_13);
    //     pin_14 = output(pin_14);
    // });

    // loop {
    //     // turn on 13,14
    //     pin_13.set_output(High);
    //     stt.wait_ticks(INITIAL_CPU_FREQ as u32);
    //     pin_14.set_output(High);
    //     stt.wait_ticks(INITIAL_CPU_FREQ as u32);

    //     // turn off 13,14
    //     pin_13.set_output(Low);
    //     stt.wait_ticks(INITIAL_CPU_FREQ as u32);
    //     pin_14.set_output(Low);
    //     stt.wait_ticks(INITIAL_CPU_FREQ as u32);
    // }
}
