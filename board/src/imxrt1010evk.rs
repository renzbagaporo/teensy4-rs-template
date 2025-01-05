//! IMXRT1010EVK board configuration.
//!
//! Peripheral pins and instances are documented inline.
//!
//! # `"spi"` feature
//!
//! When activated, the PWM peripheral is disabled,
//! and the SPI peripheral takes its place. When not activated,
//! the SPI peripheral is the unit type `()`.
//!
//! This board repurposes the SPI pins for PWM instead of using the
//! hardware-allocated PWM pins. Hardware-allocated PWM pins require
//! that you populate and de-populate certain resistors. Compile-time
//! configurations are faster than working with 0402 resistors.

use defmt_rtt as _;

use crate::{hal, iomuxc::imxrt1010 as iomuxc, ral};

use imxrt1010evk_fcb as _;

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio::GPIO_11>;

/// IMXRT1010EVK specific peripherals.
pub struct Specifics {
    pub led: Led,
}

impl Specifics {
    pub(crate) fn new() -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let iomuxc = super::convert_iomuxc(iomuxc);

        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
        let mut gpio1 = hal::gpio::Port::new(gpio1);

        let led = gpio1.output(iomuxc.gpio.p11);

        Self {
            led,
        }
    }
}
