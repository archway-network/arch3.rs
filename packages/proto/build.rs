use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process,
};

use error_chain::error_chain;
use glob::glob;

const ARCHWAY_DIR: &str = "archway-network";

const OUT_DIR: &str = "src/proto";

const EXCLUDED_PROTO_PACKAGES: &[&str] = &["google"];

error_chain! {
    foreign_links {
        IoError(io::Error);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
        Utf8Error(std::str::Utf8Error);
    }
}

fn main() -> Result<()> {
    let archway_dir = format!("{}/{}", workspace_root()?, ARCHWAY_DIR);
    let archway_proto_dir = format!("{}/proto", archway_dir);

    let protos = collect_protos(archway_proto_dir.as_str())?;
    let includes = &[
        archway_proto_dir,
        format!("{}/third_party/proto", archway_dir),
    ];

    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .extern_path(".cosmos", "::cosmos_sdk_proto::cosmos")
        .out_dir(OUT_DIR)
        .compile(protos.as_slice(), includes)?;

    let rev = git_rev(archway_dir.as_str())?;
    output_protocol_version(OUT_DIR, rev.as_str())?;
    cleanup(OUT_DIR)?;

    Ok(())
}

fn workspace_root() -> Result<String> {
    let output = run_cmd(
        env!("CARGO"),
        ["locate-project", "--workspace", "--message-format=plain"],
    )?;
    let cargo_path = Path::new(&output);
    let workspace_root = cargo_path.parent().unwrap();
    Ok(workspace_root.to_string_lossy().to_string())
}

fn collect_protos(proto_dir: &str) -> Result<Vec<PathBuf>> {
    let proto_glob = format!("{proto_dir}/**/*.proto");
    let protos: Vec<PathBuf> = glob(proto_glob.as_str())?.flatten().collect();
    Ok(protos)
}

fn git_rev(archway_dir: &str) -> Result<String> {
    let rev = run_cmd("git", ["-C", archway_dir, "describe", "--tags"])?;
    Ok(rev)
}

fn output_protocol_version(out_dir: &str, rev: &str) -> Result<()> {
    let path = Path::new(out_dir).join("ARCHWAY_COMMIT");
    fs::write(path, rev)?;
    Ok(())
}

fn cleanup(out_dir: &str) -> Result<()> {
    for pkg in EXCLUDED_PROTO_PACKAGES {
        let excluded_files_glob = format!("{out_dir}/{pkg}.*.rs");
        glob(excluded_files_glob.as_str())?
            .flatten()
            .try_for_each(fs::remove_file)?;
    }
    Ok(())
}

fn run_cmd(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<String> {
    let process::Output { stdout, status, .. } = process::Command::new(&cmd)
        .args(args)
        .output()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !status.success() {
        panic!(
            "{:?} exited with error code: {:?}",
            cmd.as_ref(),
            status.code()
        );
    }

    let output = std::str::from_utf8(&stdout)?.trim();
    Ok(output.to_string())
}
