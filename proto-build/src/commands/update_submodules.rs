use crate::consts::{ARCHWAY_DIR, ARCHWAY_REV, WASMD_DIR, WASMD_REV};
use crate::utils::run::run_git;
use std::path::Path;

pub fn update_submodules(submodules_dir: &Path) {
    run_git(["submodule", "update", "--init", "--remote", "--checkout"]).unwrap();

    println!("Updating archway-network/archway submodule...");
    let archway_dir = submodules_dir.join(ARCHWAY_DIR);
    run_git([
        "-C",
        archway_dir.to_str().unwrap(),
        "reset",
        "--hard",
        ARCHWAY_REV,
    ])
    .unwrap();

    println!("Updating wasmd submodule...");
    let wasmd_dir = submodules_dir.join(WASMD_DIR);
    run_git([
        "-C",
        wasmd_dir.to_str().unwrap(),
        "reset",
        "--hard",
        WASMD_REV,
    ])
    .unwrap();
}
