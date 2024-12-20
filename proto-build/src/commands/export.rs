use crate::consts::{ARCHWAY_DIR, WASMD_DIR};
use crate::utils::run::run_buf_export;
use std::fs;
use std::path::Path;

pub fn export(submodules_dir: &Path, proto_dir: &Path) {
    if proto_dir.exists() {
        fs::remove_dir_all(proto_dir).unwrap();
    }

    run_buf_export(submodules_dir, WASMD_DIR, proto_dir).unwrap();
    run_buf_export(submodules_dir, ARCHWAY_DIR, proto_dir).unwrap();
}
