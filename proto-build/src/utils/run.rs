use std::ffi::OsStr;
use std::path::Path;
use std::{io, process};

pub fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> crate::Result<String> {
    run_cmd("git", args)
}

pub fn run_cargo(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> crate::Result<String> {
    run_cmd(env!("CARGO"), args)
}

pub fn run_buf_export(
    submodules_dir: &Path,
    proto: &str,
    export_dir: &Path,
) -> crate::Result<String> {
    println!("Exporting {}...", proto);
    let proto_path = submodules_dir.join(proto).join("proto");
    run_cmd(
        "buf",
        [
            "export",
            "-o",
            &export_dir.display().to_string(),
            proto_path.to_str().unwrap(),
        ],
    )
}

pub fn run_cmd(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> crate::Result<String> {
    let process::Output {
        stdout,
        stderr,
        status,
    } = process::Command::new(&cmd)
        .args(args)
        .output()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    let output = std::str::from_utf8(&stdout)?.trim();
    if !status.success() {
        let error = std::str::from_utf8(&stderr)?.trim();
        panic!(
            "{:?} exited with error code: {:?}\nstdout: {:?}\nstderr: {:?}",
            cmd.as_ref(),
            status.code().unwrap_or(-1),
            output,
            error
        );
    }

    Ok(output.to_string())
}
