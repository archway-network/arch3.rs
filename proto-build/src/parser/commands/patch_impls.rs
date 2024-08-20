use std::collections::BTreeMap;
use syn::{AngleBracketedGenericArguments, File, GenericArgument, Item, PathArguments, Type, TypePath};
use syn::punctuated::Punctuated;
use crate::parser::consts::GENERICS;
use crate::parser::utils::common::{type_as_path, item_as_struct, gen_generic};

pub fn patch_impls(files: &mut BTreeMap<String, (File, BTreeMap<String, usize>)>) {
    for (_, (ast, structs)) in files.iter_mut() {
        // Fix the Name implementations
        // Since we are borrowing from the same array, we need to split the list to use it
        for i in 1..ast.items.len() {
            let (left, right) = ast.items.split_at_mut(i);
            let item = right.get_mut(0).unwrap();
            if let Item::Impl(impl_item) = item {
                // Ignore if its not the Name impl
                if let Some(t) = impl_item.trait_.clone() {
                    if t.1.segments.last().unwrap().ident != "Name" {
                        continue;
                    }
                }

                // Get the struct that is getting an impl
                let implemented = type_as_path(impl_item.self_ty.as_mut())
                    .map(|p| p.path.segments.last().unwrap().ident.to_string())
                    .unwrap_or("".to_string());

                let idx = match structs.get(&implemented) {
                    // If its not a struct then ignore
                    None => continue,
                    Some(idx) => *idx,
                };
                // Unwrap cause we know its a struct
                let struct_item = item_as_struct(left.get_mut(idx).unwrap()).unwrap();
                // Ignore structs with no generics
                if struct_item.generics.params.is_empty() {
                    continue;
                }

                impl_item.generics = struct_item.generics.clone();

                if let Some(impl_path) = type_as_path(impl_item.self_ty.as_mut()) {
                    let mut args = Punctuated::new();

                    for gen in GENERICS[0..impl_item.generics.params.len()].iter() {
                        args.push(GenericArgument::Type(Type::Path(TypePath {
                            qself: None,
                            path: gen_generic(gen),
                        })));
                    }

                    let seg = impl_path.path.segments.last_mut().unwrap();

                    seg.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args,
                        gt_token: Default::default(),
                    });
                }
            }
        }
    }
}
