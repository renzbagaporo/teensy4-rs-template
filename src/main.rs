//! Simply turns on the LED.

#![no_main]
#![no_std]

use imxrt_ral as ral;
use imxrt_iomuxc as iomuxc;
use imxrt_hal as hal;

use panic_halt as _;

use iomuxc::imxrt1010::Pads;

pub mod fcb;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

#[imxrt_rt::entry]
fn main() -> ! {

    let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
    let iomuxc = convert_iomuxc(iomuxc);

    let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
    let mut gpio1 = hal::gpio::Port::new(gpio1);

    let led = gpio1.output(iomuxc.gpio.p11);


    let mut on = false;
    loop {
        on = !on;
        if on {
            led.set();
        } else {
            led.clear();
        }

        cortex_m::asm::delay(1000000000);
    }
}

