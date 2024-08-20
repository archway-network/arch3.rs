use std::path::{Path, PathBuf};
use glob::glob;
use crate::utils::run::run_cmd;

fn collect_files(dir: &Path, pattern: &str) -> crate::Result<Vec<PathBuf>> {
    // dir.join("**").join(pattern);
    let file_glob = format!("{}/**/{pattern}", dir.display());
    let paths: Vec<PathBuf> = glob(file_glob.as_str())?.flatten().collect();
    Ok(paths)
}

pub fn rustfmt(out_dir: &Path) -> crate::Result<String> {
    println!("Running rustfmt...");
    let files = collect_files(out_dir, "*.rs")?.into_iter().map(Into::into);
    let args: Vec<std::ffi::OsString> = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .chain(files)
        .collect();

    run_cmd("rustfmt", args)
}
