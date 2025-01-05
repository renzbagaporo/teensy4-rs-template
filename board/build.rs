//! This build script prepares the HAL build.

use imxrt_rt::{Family, Memory, RuntimeBuilder};
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from)?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::write(out_dir.join("device.x"), DEVICE_X)?;

    RuntimeBuilder::from_flexspi(Family::Imxrt1010, 16 * 1024 * 1024)
        .flexram_banks(imxrt_rt::FlexRamBanks {
            ocram: 1,
            itcm: 2,
            dtcm: 1,
        })
        .uninit(Memory::Dtcm)
        .stack_size(16 * 1024)
        .build()?;
    println!("cargo:rustc-cfg=board=\"imxrt1010evk\"");
    println!("cargo:rustc-cfg=chip=\"imxrt1010\"");
    return Ok(());
}

const DEVICE_X: &str = r#"
"#;
