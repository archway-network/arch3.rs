mod commands;
mod utils;
mod consts;

use std::path::Path;
use crate::parser::commands::{
    load_and_patch_any::load_and_patch_any,
    patch_generics::patch_generics,
    patch_impls::patch_impls,
    save::save,
};

pub fn generate_advanced_struct(out_dir: &Path) -> crate::Result<()> {
    println!("Loading and patching all files containing Any");
    let mut project_tokens = load_and_patch_any(out_dir);
    println!("Patching generic trait constraints");
    patch_generics(&mut project_tokens);
    println!("Patching prost Name impls");
    patch_impls(&mut project_tokens);
    println!("Saving changes");
    save(out_dir, &project_tokens);

    Ok(())
}
