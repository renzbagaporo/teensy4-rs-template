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
        clock_gate, perclk_clk,
    },
    ral::ccm::CCM,
    RunMode,
};

pub(crate) const fn ipg_divider(run_mode: RunMode) -> u32 {
    match run_mode {
        RunMode::Overdrive => 4,
    }
}
