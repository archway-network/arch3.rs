use crate::consts::{ARCHWAY_REV, WASMD_REV};
use std::fs;
use std::path::Path;

pub fn output_versions(out_dir: &Path) {
    println!("Writing versions...");
    let out_dir = Path::new(out_dir);
    fs::write(out_dir.join("ARCHWAY_COMMIT"), ARCHWAY_REV).unwrap();
    fs::write(out_dir.join("WASMD_COMMIT"), WASMD_REV).unwrap();
}
