use std::fs;
use std::path::Path;
use crate::consts::{ARCHWAY_REV, COSMOS_SDK_REV, IBC_REV, WASMD_REV};

pub fn output_versions(out_dir: &Path) {
    println!("Writing versions...");
    let out_dir = Path::new(out_dir);
    fs::write(out_dir.join("ARCHWAY_COMMIT"), ARCHWAY_REV).unwrap();
    fs::write(out_dir.join("COSMOS_SDK_COMMIT"), COSMOS_SDK_REV).unwrap();
    fs::write(out_dir.join("IBC_COMMIT"), IBC_REV).unwrap();
    fs::write(out_dir.join("WASMD_COMMIT"), WASMD_REV).unwrap();
}
