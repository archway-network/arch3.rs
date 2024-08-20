use std::path::{Path, PathBuf};
use glob::glob;
use crate::utils::patch_file::patch_file;

/// Fix clashing type names in prost-generated code.
fn apply_cosmos_staking_patches(out_dir: &Path) {
    const REPLACEMENTS: &[(&str, &str)] = &[
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ];

    patch_file(&out_dir.join("cosmos.staking.v1beta1.rs"), REPLACEMENTS)
        .expect("error patching cosmos.staking.v1beta1.rs");
}

pub fn apply_patches(out_dir: &Path) -> crate::Result<()> {
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

    let src_files_glob = out_dir.join("*.rs");
    let src_files: Vec<PathBuf> = glob(src_files_glob.to_str().unwrap())?.flatten().collect();
    for src in src_files {
        patch_file(&src, REPLACEMENTS)?;
    }

    apply_cosmos_staking_patches(out_dir);

    Ok(())
}
