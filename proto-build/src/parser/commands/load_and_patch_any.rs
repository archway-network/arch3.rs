use crate::parser::consts::GENERICS;
use crate::parser::utils::common::{fields_as_named, item_as_struct};
use crate::parser::utils::gen_type_param::{gen_any, gen_type_param};
use crate::parser::utils::is_important::is_important;
use glob::glob;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, GenericParam, Item};

pub fn load_and_patch_any(out_dir: &Path) -> BTreeMap<String, (File, BTreeMap<String, usize>)> {
    // Map all file ASTs
    let mut project_tokens = BTreeMap::new();

    // Get all generated files
    let src_files_glob = out_dir.join("*.rs");
    let src_files: Vec<PathBuf> = glob(src_files_glob.to_str().unwrap())
        .unwrap()
        .flatten()
        .collect();

    for src in src_files {
        let current_file = fs::read_to_string(&src).unwrap();

        let mut ast = syn::parse_file(&current_file).unwrap();

        // Get all struct we might work with here
        let mut structs = BTreeMap::new();

        // First pass is just for replacing Any
        // Also adds all the serde related information to serialize and deserialize into an appropriate structure
        for (idx, item) in ast.items.iter_mut().enumerate() {
            if let Some(item) = item_as_struct(item) {
                // Find any fields and replace with generics
                let fields = fields_as_named(&mut item.fields).unwrap();
                for field in fields.named.iter_mut() {
                    if let Some((_, path)) = is_important(field) {
                        if path.path.segments.last().unwrap().ident == "Any" {
                            // Set struct generics
                            let generic = GENERICS[item.generics.params.len()];
                            path.path = gen_any(generic);
                            item.generics
                                .params
                                .push(GenericParam::Type(gen_type_param(generic)));
                        }
                    }
                }

                // Add the struct reference to work on it later
                structs.insert(item.ident.to_string(), idx);
            }
        }

        // Remove the tonic include for now
        if let Some(Item::Macro(_)) = ast.items.last() {
            ast.items.pop();
        }

        let file_name = src.to_str().unwrap().split('/').last().unwrap();
        project_tokens.insert(file_name.to_string(), (ast, structs));
    }

    project_tokens
}
