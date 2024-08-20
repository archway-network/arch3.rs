use std::path::Path;
use crate::{ARCHWAY_DIR, ARCHWAY_REV, COSMOS_SDK_DIR, COSMOS_SDK_REV, IBC_DIR, IBC_REV, WASMD_DIR, WASMD_REV};
use crate::utils::run::run_git;

pub fn update_submodules(submodules_dir: &Path) {
    run_git(["submodule", "update", "--init"]).unwrap();
    run_git(["submodule", "foreach", "git", "fetch"]).unwrap();

    println!("Updating archway-network/archway submodule...");
    let archway_dir = format!("{}/{}", submodules_dir.display(), ARCHWAY_DIR);
    run_git(["-C", archway_dir.as_str(), "reset", "--hard", ARCHWAY_REV]).unwrap();

    println!("Updating cosmos/cosmos-sdk submodule...");
    let sdk_dir = format!("{}/{}", submodules_dir.display(), COSMOS_SDK_DIR);
    run_git(["-C", sdk_dir.as_str(), "reset", "--hard", COSMOS_SDK_REV]).unwrap();

    println!("Updating cosmos/ibc-go submodule...");
    let ibc_dir = format!("{}/{}", submodules_dir.display(), IBC_DIR);
    run_git(["-C", ibc_dir.as_str(), "reset", "--hard", IBC_REV]).unwrap();

    println!("Updating wasmd submodule...");
    let wasmd_dir = format!("{}/{}", submodules_dir.display(), WASMD_DIR);
    run_git(["-C", wasmd_dir.as_str(), "reset", "--hard", WASMD_REV]).unwrap();
}
