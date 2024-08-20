use std::{fs, io};
use std::path::Path;
use regex::Regex;

pub fn patch_file(path: &Path, replacements: &[(&str, &str)]) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;

    for &(regex, replacement) in replacements {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(path, &contents)
}
