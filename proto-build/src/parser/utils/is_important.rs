use crate::parser::utils::common::type_as_path;
use syn::{Field, GenericArgument, PathArguments, TypePath};

pub enum FoundEnclosure {
    Option,
    Vec,
}

// The types we're going to be modifying only appear enclosed in Options and Vecs
pub fn is_important(field: &mut Field) -> Option<(FoundEnclosure, &mut TypePath)> {
    let path = type_as_path(&mut field.ty).unwrap();
    // Get the last segment since the rest is a path
    let field_type = path.path.segments.iter_mut().last().unwrap();

    let field_type_ident = field_type.ident.to_string();
    let found = if &field_type_ident == "Option" {
        Some(FoundEnclosure::Option)
    } else if &field_type_ident == "Vec" {
        Some(FoundEnclosure::Vec)
    } else {
        None
    };

    // If we found the types we needed
    if let Some(enclosed) = found {
        if let PathArguments::AngleBracketed(bracket) = &mut field_type.arguments {
            for p in bracket.args.iter_mut() {
                if let GenericArgument::Type(t) = p {
                    if let Some(any_path) = type_as_path(t) {
                        return Some((enclosed, any_path));
                    } else {
                        println!("something else")
                    }
                }
            }
        }
    }

    None
}
