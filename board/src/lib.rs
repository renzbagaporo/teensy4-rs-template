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

mod ral_shim;

/// SOC run mode.
///
/// Each MCU specifies its own core clock speed
/// and power settings for these variants. They're
/// typically follow the recommendations in the
/// data sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[non_exhaustive]
pub enum RunMode {
    /// The fastest, highest-power mode.
    Overdrive,
}

pub use ral_shim::{BOARD_DMA_A_INDEX, BOARD_DMA_B_INDEX, NVIC_PRIO_BITS};

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
    /// PIT channels.
    pub pit: hal::pit::Channels,
    /// GPT1 timer.
    ///
    /// Use [`GPT1_FREQUENCY`] to understand its frequency.
    pub gpt1: hal::gpt::Gpt<1>,
    /// GPT2 timer.
    ///
    /// Use [`GPT2_FREQUENCY`] to understand its frequency.
    pub gpt2: hal::gpt::Gpt<2>,
    /// DMA channels.
    pub dma: [Option<hal::dma::channel::Channel>; hal::dma::CHANNEL_COUNT],
    /// Secure real-time counter.
    ///
    /// Examples may enable the SRTC.
    pub srtc: hal::snvs::srtc::Disabled,
    /// SNVS LP core registers.
    ///
    /// May be used with the SRTC.
    pub snvs_lp_core: hal::snvs::LpCore,
}

impl Common {
    /// Prepares common resources.
    fn new() -> Self {
        let pit: Pit = unsafe { Pit::instance() };
        // Stop timers in debug mode.
        ral::modify_reg!(ral::pit, pit, MCR, FRZ: FRZ_1);
        let pit = hal::pit::new(pit);

        let gpt1 = configure_gpt(unsafe { ral::gpt::GPT1::instance() }, GPT1_DIVIDER);
        let gpt2 = configure_gpt(unsafe { ral::gpt::GPT2::instance() }, GPT2_DIVIDER);

        let dma = hal::dma::channels(unsafe { ral::dma::DMA::instance() }, unsafe {
            ral::dmamux::DMAMUX::instance()
        });

        let hal::snvs::Snvs {
            low_power:
                hal::snvs::LowPower {
                    core: snvs_lp_core,
                    srtc,
                    ..
                },
            ..
        } = hal::snvs::new(unsafe { ral::snvs::SNVS::instance() });

        Self {
            pit,
            gpt1,
            gpt2,
            dma,
            srtc,
            snvs_lp_core,
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
    unsafe {
        ral_shim::shim_vectors();
        configure();
        let mut common = Common::new();
        let specifics = Specifics::new(&mut common);
        (common, specifics)
    }
}

/// The board's run mode.
pub const RUN_MODE: RunMode = RunMode::Overdrive;

const GPT1_DIVIDER: u32 = 10;
const GPT2_DIVIDER: u32 = 100;
const GPT_SELECTION: hal::gpt::ClockSource = hal::gpt::ClockSource::HighFrequencyReferenceClock;


use iomuxc::imxrt1010::Pads;

/// Convert the IOMUXC peripheral into pad objects.
fn convert_iomuxc(_: ral::iomuxc::IOMUXC) -> Pads {
    // Safety: acquired IOMUXC peripheral, so no one else is safely
    // using this peripheral.
    unsafe { Pads::new() }
}

fn configure_gpt<const N: u8>(gpt: ral::gpt::Instance<N>, divider: u32) -> hal::gpt::Gpt<N>
where
    ral::gpt::Instance<N>: ral::Valid,
{
    let mut gpt = hal::gpt::Gpt::new(gpt);
    gpt.disable();
    gpt.set_wait_mode_enable(true);
    gpt.set_clock_source(GPT_SELECTION);
    gpt.set_divider(divider);
    gpt
}


type Pit = crate::ral::pit::PIT;

/// Board interrupts.
///
/// Associated to interrupt numbers in board modules.
#[allow(unused)]
mod board_interrupts {
    pub type Vector = unsafe extern "C" fn();
    extern "C" {
        pub fn BOARD_CONSOLE();
        pub fn BOARD_BUTTON();
        pub fn BOARD_DMA_A();
        pub fn BOARD_DMA_B();
        pub fn BOARD_PIT();
        pub fn BOARD_GPT1();
        pub fn BOARD_GPT2();
    }
}

/// A simple blocking executor for async hardware examples.
pub mod blocking {
    use core::{future::Future, pin::Pin, task::Poll};

    /// Poll a future with a dummy waker.
    ///
    /// Use `poll_no_wake` when you want to drive a future to completion, but you
    /// don't care about the future waking an executor. It may be used to initiate
    /// a DMA transfer that will later be awaited with [`block`].
    ///
    /// Do not use `poll_no_wake` if you want an executor to be woken when the DMA
    /// transfer completes.
    fn poll_no_wake<F>(future: Pin<&mut F>) -> Poll<F::Output>
    where
        F: Future,
    {
        use core::task::{Context, RawWaker, RawWakerVTable, Waker};
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});

        const RAW_WAKER: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
        // Safety: raw waker meets documented requirements.
        let waker = unsafe { Waker::from_raw(RAW_WAKER) };
        let mut context = Context::from_waker(&waker);
        future.poll(&mut context)
    }

    /// Block until the future returns a result.
    ///
    /// `block` invokes `poll_no_wake()` in a loop until the future
    /// returns a result. Consider using `block` after starting a transfer
    /// with `poll_no_wake`, and after doing other work.
    pub fn run<F>(mut future: Pin<&mut F>) -> F::Output
    where
        F: Future,
    {
        loop {
            match poll_no_wake(future.as_mut()) {
                Poll::Ready(result) => return result,
                Poll::Pending => {}
            }
        }
    }
}
