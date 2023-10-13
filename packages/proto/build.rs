use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process,
};

use error_chain::error_chain;
use glob::glob;
use regex::Regex;

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
    compile_proto(archway_dir.as_str())?;
    cleanup(OUT_DIR)?;
    apply_patches(OUT_DIR)?;
    output_protocol_version(OUT_DIR, archway_dir.as_str())?;
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

fn compile_proto(archway_dir: &str) -> Result<()> {
    let archway_proto_dir = format!("{}/proto", archway_dir);
    let protos = collect_proto_files(archway_proto_dir.as_str())?;
    let includes = &[
        archway_proto_dir,
        format!("{}/third_party/proto", archway_dir),
    ];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .extern_path(".cosmos", "::cosmos_sdk_proto::cosmos")
        .extern_path(".tendermint", "::cosmos_sdk_proto::tendermint")
        .client_mod_attribute(".", "#[cfg(feature = \"grpc\")]")
        .client_mod_attribute(".", "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]")
        .server_mod_attribute(".", "#[cfg(feature = \"grpc\")]")
        .server_mod_attribute(".", "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]")
        .out_dir(OUT_DIR)
        .compile(protos.as_slice(), includes)?;

    Ok(())
}

fn collect_proto_files(proto_dir: &str) -> Result<Vec<PathBuf>> {
    let proto_glob = format!("{proto_dir}/**/*.proto");
    let protos: Vec<PathBuf> = glob(proto_glob.as_str())?.flatten().collect();
    Ok(protos)
}

fn apply_patches(out_dir: &str) -> Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
    ];

    let src_files_glob = format!("{out_dir}/*.rs");
    let src_files: Vec<PathBuf> = glob(src_files_glob.as_str())?.flatten().collect();
    for src in src_files {
        let mut contents = fs::read_to_string(src.as_path())?;

        for &(regex, replacement) in REPLACEMENTS {
            contents = Regex::new(regex)
                .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
                .replace_all(&contents, replacement)
                .to_string();
        }

        fs::write(src, &contents)?;
    }

    Ok(())
}

fn output_protocol_version(out_dir: &str, archway_dir: &str) -> Result<()> {
    let git_rev = run_cmd("git", ["-C", archway_dir, "describe", "--tags"])?;
    let path = Path::new(out_dir).join("ARCHWAY_COMMIT");
    fs::write(path, git_rev)?;
    Ok(())
}

fn cleanup(out_dir: &str) -> Result<()> {
    for &pkg in EXCLUDED_PROTO_PACKAGES {
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
