mod parser;
mod utils;
mod commands;

use std::{io, path::Path, process};
use std::path::PathBuf;

use crate::parser::generate_advanced_struct;
use error_chain::error_chain;
use crate::commands::apply_patches::apply_patches;
use crate::commands::cleanup::cleanup;
use crate::commands::export::export;
use crate::commands::generate::generate;
use crate::commands::output_version::output_versions;
use crate::commands::rustfmt::rustfmt;
use crate::commands::update_submodules::update_submodules;
use crate::utils::run::run_cargo;

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
