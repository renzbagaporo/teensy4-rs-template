//! A thin board support package for `imxrt-hal` hardware examples.
//!
//! The top-level module exposes configurations and APIs that are common across
//! boards. For board specific information, like which LPUART is the console and
//! which pins are I2C, see the board-specific modules.

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use imxrt_hal as hal;
use imxrt_iomuxc as iomuxc;
use imxrt_ral as ral;
use imxrt_rt as _;

#[path = "imxrt1010evk.rs"]
mod board_impl;

// rustdoc doesn't like when this is named 'board'
// since it happens to match the package name.
// So went with an '_impl' suffix.
pub use board_impl::*;

/// Components that are common to all boards.
///
/// This includes timers, DMA channels, and things
/// that don't necessarily depend on a pinout.
pub struct Common {
}

impl Common {
    /// Prepares common resources.
    fn new() -> Self {
        Self {
        }
    }
}

/// Board entrypoint.
///
/// Use this to configure the hardware and acquire peripherals.
///
/// # Panics
///
/// This should only be called once, at the top of your `main()` routine.
/// It panics if any hardware resource is already taken.
pub fn new() -> (Common, Specifics) {
    static ONCE: AtomicBool = AtomicBool::new(false);
    let done = ONCE.fetch_or(true, Ordering::SeqCst);
    assert!(!done, "You've already initialized the board.");

    // Safety: once flag ensures that this only happens once.
    let mut common = Common::new();
    let specifics = Specifics::new(&mut common);
    (common, specifics)
}


use iomuxc::imxrt1010::Pads;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

/// Board interrupts.
///
/// Associated to interrupt numbers in board modules.
#[allow(unused)]
mod board_interrupts {
    pub type Vector = unsafe extern "C" fn();
    extern "C" {
        pub fn BOARD_BUTTON();
    }
}