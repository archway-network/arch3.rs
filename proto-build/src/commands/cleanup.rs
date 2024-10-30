use glob::glob;
use std::fs;
use std::path::Path;

const EXCLUDED_PROTO_PACKAGES: &[&str] = &["amino", "gogoproto", "google", "tendermint"];

pub fn cleanup(out_dir: &Path) {
    println!("Cleaning up...");
    for &pkg in EXCLUDED_PROTO_PACKAGES {
        let excluded_files_glob = format!("{}/{pkg}*.rs", out_dir.display());
        glob(excluded_files_glob.as_str())
            .unwrap()
            .flatten()
            .try_for_each(fs::remove_file)
            .unwrap();
    }
}
