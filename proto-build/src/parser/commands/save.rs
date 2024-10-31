use crate::utils::patch_file::patch_file;
use quote::quote;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use syn::File;

pub fn save(out_dir: &Path, files: &BTreeMap<String, (File, BTreeMap<String, usize>)>) {
    for (file, (data, _)) in files.iter() {
        // Patch the mod file
        let file_regex = Regex::new(r"(\.[[:alnum:]]+\.)rs").unwrap();
        let new_file = file_regex.replace(file, "${1}abstract.rs").to_string();

        patch_file(
            &out_dir.join("mod.rs"),
            &[(
                &format!(
                    r"include!\(.{}.\);",
                    Regex::new(r"(\.)").unwrap().replace(file, r"\.")
                ),
                &format!(
                    "\
                #[cfg(not(feature = \"abstract-any\"))]\n\
                include!(\"{}\");\n\
                #[cfg(feature = \"abstract-any\")]\n\
                include!(\"{}\");\
                ",
                    file, new_file
                ),
            )],
        )
        .unwrap();

        // Export the generated structure and save in file
        fs::write(out_dir.join(new_file), quote!(#data).to_string()).unwrap();
    }
}
