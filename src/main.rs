//! Simply turns on the LED.

#![no_main]
#![no_std]

use imxrt_ral as ral;
use cortex_m::asm;

use panic_halt as _; 

#[cortex_m_rt::entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}

