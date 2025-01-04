//! Preset clock tree configurations and frequencies.
//!
//! Use `configure` to simply configure the clock tree for a given
//! run mode. After `configure`, the system clocks run at the frequencies
//! described by each `*_frequency` function. The frequencies for a given
//! run mode are less than or equal to the maximum allowed for the given
//! run mode. Consult your MCU's reference manual for more information.
//!
//! Use `*_frequency` functions to understand the target system clock frequencies.
//! Note that these functions are `const`, and should be usable in constant
//! contexts.

pub(crate) use super::ahb::{ahb_frequency, configure_ahb_ipg};
use crate::{
    hal::ccm::{
        analog, clock_gate, lpi2c_clk, lpspi_clk, perclk_clk, XTAL_OSCILLATOR_HZ,
    },
    ral::ccm::CCM,
    RunMode,
};

pub(crate) const fn ipg_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 4,
    }
}

/// Returns the target IPG frequency (Hz) for the run mode.
pub const fn ipg_frequency(run_mode: RunMode) -> u32 {
    ahb_frequency(run_mode) / ipg_divider(run_mode)
}

const fn perclk_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 2,
    }
}

const fn perclk_selection(run_mode: RunMode) -> perclk_clk::Selection {
    match run_mode {
        RunMode::Overdrive => perclk_clk::Selection::Ipg,
    }
}

/// Returns the target PERCLK frequency (Hz) for the run mode.
pub const fn perclk_frequency(run_mode: RunMode) -> u32 {
    let hz = match run_mode {
        RunMode::Overdrive => ipg_frequency(run_mode),
    };
    hz / perclk_divider(run_mode)
}


/// Configure the PERCLK root clock.
///
/// When this call returns, the PERCLK clock frequency match the values
/// returned by the [`perclk_frequency()`] function.
///
/// This function will disable the clock gates for various peripherals. It
/// may leave these clock gates disabled.
pub fn configure_perclk(run_mode: RunMode, ccm: &mut CCM) {
    clock_gate::PERCLK_CLOCK_GATES
        .iter()
        .for_each(|locator| locator.set(ccm, clock_gate::OFF));
    perclk_clk::set_selection(ccm, perclk_selection(run_mode));
    perclk_clk::set_divider(ccm, perclk_divider(run_mode));
}