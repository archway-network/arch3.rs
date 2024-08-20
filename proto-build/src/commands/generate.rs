use std::fs;
use std::path::Path;
use crate::utils::run::run_cmd;

pub fn generate(
    buf_gen_path: &Path,
    proto_path: &Path,
    out_dir: &Path,
) -> crate::Result<String> {
    println!("Generating proto...");

    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).unwrap();
    }

    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            &buf_gen_path.display().to_string(),
            "--include-imports",
            "-o",
            &out_dir.display().to_string(),
            &proto_path.display().to_string(),
        ],
    )
}
