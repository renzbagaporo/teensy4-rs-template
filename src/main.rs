//! Simply turns on the LED.

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;

use panic_halt as _; 

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}

