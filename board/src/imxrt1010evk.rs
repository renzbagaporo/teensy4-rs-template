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

#[cfg(target_arch = "arm")]
use imxrt1010evk_fcb as _;

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

mod imxrt10xx {
    pub mod clock;
    pub mod power;

    #[path = "clock_tree/pll6_ahb.rs"]
    mod ahb;

    mod clock_tree;
}

pub use imxrt10xx::{power::*};

/// The board LED.
pub type Led = hal::gpio::Output<iomuxc::gpio::GPIO_11>;

/// The SW4 "user" button.
pub type Button = hal::gpio::Input<ButtonPad>;
type ButtonPad = iomuxc::gpio_sd::GPIO_SD_05;

/// Test point 34.
///
/// Use this for measuring your application timing (as a GPIO).
/// Or, use it to evaluate clocks via `CCM_CLKO1`.
pub type Tp34 = iomuxc::gpio_sd::GPIO_SD_02;

/// Test point 31.
///
/// Use this for measuring your application timing (as a GPIO).
/// Or, use it to evaluate clocks via `CCM_CLKO2`.
pub type Tp31 = iomuxc::gpio_sd::GPIO_SD_01;

/// Opaque structure for managing GPIO ports.
///
/// Exposes methods to configure your board's specific GPIOs.
pub struct GpioPorts {
    gpio2: hal::gpio::Port<2>,
}

impl GpioPorts {
    /// Returns the GPIO port for the button.
    pub fn button_mut(&mut self) -> &mut hal::gpio::Port<2> {
        &mut self.gpio2
    }
}

/// IMXRT1010EVK specific peripherals.
pub struct Specifics {
    pub led: Led,
    pub button: Button,
    pub ports: GpioPorts,
    pub tp34: Tp34,
    pub tp31: Tp31,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        let iomuxc = unsafe { ral::iomuxc::IOMUXC::instance() };
        let mut iomuxc = super::convert_iomuxc(iomuxc);
        configure_pins(&mut iomuxc);

        let gpio1 = unsafe { ral::gpio::GPIO1::instance() };
        let mut gpio1 = hal::gpio::Port::new(gpio1);
        let gpio2 = unsafe { ral::gpio::GPIO2::instance() };
        let mut gpio2 = hal::gpio::Port::new(gpio2);

        let led = gpio1.output(iomuxc.gpio.p11);
        let button = gpio2.input(iomuxc.gpio_sd.p05);

        Self {
            led,
            button,
            ports: GpioPorts { gpio2 },
            tp34: iomuxc.gpio_sd.p02,
            tp31: iomuxc.gpio_sd.p01,
        }
    }
}

use hal::ccm::clock_gate;

/// The clock gates for this board's peripherals.
pub(crate) const CLOCK_GATES: &[clock_gate::Locator] = &[
    clock_gate::gpio::<1>(),
];

/// Configure board pins.
///
/// Peripherals are responsible for pin muxing, so there's no need to
/// set alternates here.
fn configure_pins(
    super::Pads {
        ref mut gpio_sd,
        ..
    }: &mut super::Pads,
) {
    use crate::iomuxc;

    const BUTTON_CONFIG: iomuxc::Config = iomuxc::Config::zero()
        .set_pull_keeper(Some(iomuxc::PullKeeper::Pullup100k))
        .set_hysteresis(iomuxc::Hysteresis::Enabled);

    let button: &mut ButtonPad = &mut gpio_sd.p05;
    iomuxc::configure(button, BUTTON_CONFIG);

    // Set the pin muxing for the two test points.
    crate::iomuxc::ccm::prepare(&mut gpio_sd.p01);
    crate::iomuxc::ccm::prepare(&mut gpio_sd.p02);
}


pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_BUTTON: Interrupt = Interrupt::GPIO2_COMBINED_0_15;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_BUTTON, syms::BOARD_BUTTON),
    ];
}
pub use interrupt as Interrupt;
