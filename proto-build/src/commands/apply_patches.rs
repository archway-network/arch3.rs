use crate::utils::patch_file::patch_file;
use glob::glob;
use std::path::{Path, PathBuf};

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

    Ok(())
}
