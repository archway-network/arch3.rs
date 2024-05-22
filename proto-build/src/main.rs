use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process,
};

use error_chain::error_chain;
use glob::glob;
use regex::Regex;

/// The Archway commit or tag to be cloned and used to build the proto files
const ARCHWAY_REV: &str = "v7.0.1";
const ARCHWAY_DIR: &str = "archway";

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.47.11";
const COSMOS_SDK_DIR: &str = "cosmos-sdk";

/// The Cosmos ibc-go commit or tag to be cloned and used to build the proto files
const IBC_REV: &str = "v7.4.0";
const IBC_DIR: &str = "ibc-go";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.45.0";
const WASMD_DIR: &str = "wasmd";

const PROTO_DIR: &str = "proto";
const OUT_DIR: &str = "packages/proto/src/gen";

const EXCLUDED_PROTO_PACKAGES: &[&str] = &["amino", "gogoproto", "google", "tendermint"];

error_chain! {
    foreign_links {
        IoError(io::Error);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
        Utf8Error(std::str::Utf8Error);
    }
}

fn main() {
    let root = workspace_root();
    let submodules_dir = format!("{}/{}", root, "external");
    let proto_dir = format!("{}/{}", root, PROTO_DIR);
    let out_dir = format!("{}/{}", root, OUT_DIR);

    update_submodules(submodules_dir.as_str());
    export(submodules_dir.as_str(), proto_dir.as_str());
    generate(proto_dir.as_str(), out_dir.as_str()).unwrap();
    output_versions(out_dir.as_str());
    cleanup(out_dir.as_str());
    apply_patches(out_dir.as_str()).unwrap();
    rustfmt(out_dir.as_str()).unwrap();
}

fn workspace_root() -> String {
    let output = run_cargo(["locate-project", "--workspace", "--message-format=plain"]).unwrap();
    let cargo_path = Path::new(&output);
    let workspace_root = cargo_path.parent().unwrap();
    workspace_root.to_string_lossy().to_string()
}

fn update_submodules(submodules_dir: &str) {
    run_git(["submodule", "update", "--init"]).unwrap();
    run_git(["submodule", "foreach", "git", "fetch"]).unwrap();

    println!("Updating archway-network/archway submodule...");
    let archway_dir = format!("{}/{}", submodules_dir, ARCHWAY_DIR);
    run_git(["-C", archway_dir.as_str(), "reset", "--hard", ARCHWAY_REV]).unwrap();

    println!("Updating cosmos/cosmos-sdk submodule...");
    let sdk_dir = format!("{}/{}", submodules_dir, COSMOS_SDK_DIR);
    run_git(["-C", sdk_dir.as_str(), "reset", "--hard", COSMOS_SDK_REV]).unwrap();

    println!("Updating cosmos/ibc-go submodule...");
    let ibc_dir = format!("{}/{}", submodules_dir, IBC_DIR);
    run_git(["-C", ibc_dir.as_str(), "reset", "--hard", IBC_REV]).unwrap();

    println!("Updating wasmd submodule...");
    let wasmd_dir = format!("{}/{}", submodules_dir, WASMD_DIR);
    run_git(["-C", wasmd_dir.as_str(), "reset", "--hard", WASMD_REV]).unwrap();
}

fn export(submodules_dir: &str, proto_dir: impl AsRef<Path>) {
    if proto_dir.as_ref().exists() {
        fs::remove_dir_all(&proto_dir).unwrap();
    }

    run_buf_export(submodules_dir, ARCHWAY_DIR, &proto_dir).unwrap();
    run_buf_export(submodules_dir, COSMOS_SDK_DIR, &proto_dir).unwrap();
    run_buf_export(submodules_dir, IBC_DIR, &proto_dir).unwrap();
    run_buf_export(submodules_dir, WASMD_DIR, &proto_dir).unwrap();
}

fn output_versions(out_dir: &str) {
    println!("Writing versions...");
    let out_dir = Path::new(out_dir);
    fs::write(out_dir.join("ARCHWAY_COMMIT"), ARCHWAY_REV).unwrap();
    fs::write(out_dir.join("COSMOS_SDK_COMMIT"), COSMOS_SDK_REV).unwrap();
    fs::write(out_dir.join("IBC_COMMIT"), IBC_REV).unwrap();
    fs::write(out_dir.join("WASMD_COMMIT"), WASMD_REV).unwrap();
}

fn apply_patches(out_dir: &str) -> Result<()> {
    println!("Applying patches...");
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate the ProtoBuf descriptors
        (
            "pub const FILE_DESCRIPTOR_SET",
            "#[cfg(feature = \"proto-descriptor\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"proto-descriptor\")))]\n    \
             pub const FILE_DESCRIPTOR_SET",
        ),
    ];

    let src_files_glob = format!("{out_dir}/*.rs");
    let src_files: Vec<PathBuf> = glob(src_files_glob.as_str())?.flatten().collect();
    for src in src_files {
        patch_file(src, REPLACEMENTS)?;
    }

    apply_cosmos_staking_patches(out_dir);

    Ok(())
}

/// Fix clashing type names in prost-generated code.
fn apply_cosmos_staking_patches(out_dir: &str) {
    const REPLACEMENTS: &[(&str, &str)] = &[
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ];

    patch_file(format!("{out_dir}/cosmos.staking.v1beta1.rs"), REPLACEMENTS)
        .expect("error patching cosmos.staking.v1beta1.rs");
}

fn patch_file(path: impl AsRef<Path>, replacements: &[(&str, &str)]) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;

    for &(regex, replacement) in replacements {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(path, &contents)
}

fn cleanup(out_dir: &str) {
    println!("Cleaning up...");
    for &pkg in EXCLUDED_PROTO_PACKAGES {
        let excluded_files_glob = format!("{out_dir}/{pkg}*.rs");
        glob(excluded_files_glob.as_str())
            .unwrap()
            .flatten()
            .try_for_each(fs::remove_file)
            .unwrap();
    }
}

fn run_buf_export(
    submodules_dir: &str,
    proto: &str,
    export_dir: impl AsRef<Path>,
) -> Result<String> {
    println!("Exporting {}...", proto);
    let proto_path = format!("{}/{}/{}", submodules_dir, proto, "proto");
    run_cmd(
        "buf",
        [
            "export",
            "-o",
            &export_dir.as_ref().display().to_string(),
            &proto_path,
        ],
    )
}

fn generate(proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<String> {
    println!("Generating proto...");

    if out_dir.as_ref().exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            "proto-build/buf.gen.yaml",
            "--include-imports",
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    )
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<String> {
    run_cmd("git", args)
}

fn rustfmt(out_dir: &str) -> Result<String> {
    println!("Running rustfmt...");
    let files = collect_files(out_dir, "*.rs")?.into_iter().map(Into::into);
    let args: Vec<std::ffi::OsString> = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .chain(files)
        .collect();

    run_cmd("rustfmt", args)
}

fn collect_files(dir: &str, pattern: &str) -> Result<Vec<PathBuf>> {
    let file_glob = format!("{dir}/**/{pattern}");
    let paths: Vec<PathBuf> = glob(file_glob.as_str())?.flatten().collect();
    Ok(paths)
}

fn run_cargo(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<String> {
    run_cmd(env!("CARGO"), args)
}

fn run_cmd(
    cmd: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<String> {
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
