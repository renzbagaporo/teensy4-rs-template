//! Simply turns on the LED.

#![no_main]
#![no_std]


#[imxrt_rt::entry]
fn main() -> ! {
    let (_, board::Specifics { led, .. }) = board::new();
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

