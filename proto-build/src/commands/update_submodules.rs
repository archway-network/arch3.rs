use crate::consts::{
    ARCHWAY_DIR, ARCHWAY_REV, COSMOS_SDK_DIR, COSMOS_SDK_REV, IBC_DIR, IBC_REV, WASMD_DIR,
    WASMD_REV,
};
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

    println!("Updating cosmos/cosmos-sdk submodule...");
    let sdk_dir = submodules_dir.join(COSMOS_SDK_DIR);
    run_git([
        "-C",
        sdk_dir.to_str().unwrap(),
        "reset",
        "--hard",
        COSMOS_SDK_REV,
    ])
    .unwrap();

    println!("Updating cosmos/ibc-go submodule...");
    let ibc_dir = submodules_dir.join(IBC_DIR);
    run_git(["-C", ibc_dir.to_str().unwrap(), "reset", "--hard", IBC_REV]).unwrap();

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
