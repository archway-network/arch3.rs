mod commands;
mod consts;
mod parser;
mod utils;

use std::path::PathBuf;
use std::{io, path::Path, process};

use crate::commands::apply_patches::apply_patches;
use crate::commands::cleanup::cleanup;
use crate::commands::export::export;
use crate::commands::generate::generate;
use crate::commands::output_version::output_versions;
use crate::commands::rustfmt::rustfmt;
use crate::commands::update_submodules::update_submodules;
use crate::consts::{OUT_DIR, PROTO_DIR};
use crate::parser::generate_advanced_struct;
use crate::utils::run::run_cargo;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        IoError(io::Error);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
        Utf8Error(std::str::Utf8Error);
    }
}

fn workspace_root() -> PathBuf {
    let output = run_cargo(["locate-project", "--workspace", "--message-format=plain"]).unwrap();
    let cargo_path = Path::new(&output);
    cargo_path.parent().unwrap().to_path_buf()
}

fn main() {
    let root = workspace_root();
    let submodules_dir = root.join("external");
    let proto_dir = root.join(PROTO_DIR);
    let out_dir = root.join(OUT_DIR);
    let buf_gen_path = root.join("proto-build").join("buf.gen.yaml");

    update_submodules(&submodules_dir);
    export(&submodules_dir, &proto_dir);
    generate(&buf_gen_path, &proto_dir, &out_dir).unwrap();
    output_versions(&out_dir);
    cleanup(&out_dir);
    apply_patches(&out_dir).unwrap();
    generate_advanced_struct(&out_dir).unwrap();
    rustfmt(&out_dir).unwrap();
}
