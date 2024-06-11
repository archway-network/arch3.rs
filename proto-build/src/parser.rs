use crate::patch_file;
use glob::glob;
use proc_macro2::{Literal, Punct, Spacing, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use syn::punctuated::Punctuated;
use syn::token::{Paren, PathSep};
use syn::{
    AngleBracketedGenericArguments, AttrStyle, Attribute, Field, Fields, FieldsNamed, File,
    GenericArgument, GenericParam, Ident, Item, ItemStruct, MacroDelimiter, Meta, MetaList, Path,
    PathArguments, PathSegment, TraitBound, TraitBoundModifier, Type, TypeParam, TypeParamBound,
    TypePath,
};

fn as_struct(item: &mut Item) -> Option<&mut ItemStruct> {
    match item {
        Item::Struct(s) => Some(s),
        _ => None,
    }
}

fn as_path(item: &mut Type) -> Option<&mut TypePath> {
    match item {
        Type::Path(ret) => Some(ret),
        _ => None,
    }
}

fn as_named_fields(fields: &mut Fields) -> Option<&mut FieldsNamed> {
    match fields {
        Fields::Named(ret) => Some(ret),
        _ => None,
    }
}

enum FoundEnclosure {
    Option,
    Vec,
}

// The types we're going to be modifying only appear enclosed in Options and Vecs
fn is_important(field: &mut Field) -> Option<(FoundEnclosure, &mut TypePath)> {
    let path = as_path(&mut field.ty).unwrap();
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
                    if let Some(any_path) = as_path(t) {
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

// prost Message implements a generic B which was conflicting with this script
const GENERICS: [&str; 10] = ["A", "BB", "C", "D", "E", "F", "G", "H", "I", "J"];

fn gen_generic(name: &str) -> Path {
    Path {
        leading_colon: None,
        segments: create_punctuated(vec![name]),
    }
}

fn create_punctuated(path: Vec<&str>) -> Punctuated<PathSegment, PathSep> {
    let mut ret = Punctuated::new();
    for p in path {
        ret.push(PathSegment {
            ident: Ident::new(p, Span::call_site()),
            arguments: PathArguments::None,
        });
    }
    ret
}

fn trait_param_bound(path: Vec<&str>) -> TypeParamBound {
    TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: Path {
            leading_colon: None,
            segments: create_punctuated(path),
        },
    })
}

fn gen_unnamed_param(name: &str) -> TypeParam {
    let mut type_param = TypeParam::from(Ident::new(name, Span::call_site()));
    type_param.bounds.push(trait_param_bound(vec!["Clone"]));
    type_param.bounds.push(trait_param_bound(vec!["PartialEq"]));
    type_param.bounds.push(trait_param_bound(vec!["Default"]));
    type_param.bounds.push(trait_param_bound(vec!["Send"]));
    type_param.bounds.push(trait_param_bound(vec!["Sync"]));
    type_param
        .bounds
        .push(trait_param_bound(vec!["prost", "Message"]));
    type_param
        .bounds
        .push(trait_param_bound(vec!["serde", "Serialize"]));
    type_param
        .bounds
        .push(trait_param_bound(vec!["serde", "de", "DeserializeOwned"]));

    type_param
}

fn gen_type_param(name: &str) -> TypeParam {
    let mut type_param = gen_unnamed_param(name);
    type_param
        .bounds
        .push(trait_param_bound(vec!["prost", "Name"]));
    type_param
}

pub fn generate_advanced_struct(out_dir: &str) -> crate::Result<()> {
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

fn load_and_patch_any(out_dir: &str) -> BTreeMap<String, (File, BTreeMap<String, usize>)> {
    // Map all file ASTs
    let mut project_tokens = BTreeMap::new();

    // Get all generated files
    let src_files_glob = format!("{out_dir}/*.rs");
    let src_files: Vec<PathBuf> = glob(src_files_glob.as_str()).unwrap().flatten().collect();

    for src in src_files {
        let current_file = fs::read_to_string(&src).unwrap();

        // // Filter files that dont have `Any`
        // if !any_filter.is_match(&current_file) {
        //     continue;
        // }

        let mut ast = syn::parse_file(&current_file).unwrap();

        // Get all struct we might work with here
        let mut structs = BTreeMap::new();

        // First pass is just for replacing Any
        // Also adds all the serde related information to serialize and deserialize into an appropriate structure
        for (idx, item) in ast.items.iter_mut().enumerate() {
            if let Some(item) = as_struct(item) {
                // Find any fields and replace with generics
                let fields = as_named_fields(&mut item.fields).unwrap();
                for field in fields.named.iter_mut() {
                    if let Some((ty, path)) = is_important(field) {
                        if path.path.segments.last().unwrap().ident == "Any" {
                            // Set struct generics
                            let generic = GENERICS[item.generics.params.len()];
                            path.path = gen_generic(generic);
                            item.generics
                                .params
                                .push(GenericParam::Type(gen_type_param(generic)));

                            // Set serialization function
                            let serde_path = match ty {
                                FoundEnclosure::Option => "option",
                                FoundEnclosure::Vec => "vec",
                            };

                            let mut token_stream = TokenStream::new();
                            token_stream.append(Ident::new("serialize_with", Span::call_site()));
                            token_stream.append(Punct::new('=', Spacing::Alone));
                            token_stream.append(Literal::string(&format!(
                                "crate::any::{}::serialize",
                                serde_path
                            )));
                            token_stream.append(Punct::new(',', Spacing::Alone));
                            token_stream.append(Ident::new("deserialize_with", Span::call_site()));
                            token_stream.append(Punct::new('=', Spacing::Alone));
                            token_stream.append(Literal::string(&format!(
                                "crate::any::{}::deserialize",
                                serde_path
                            )));

                            field.attrs.push(Attribute {
                                pound_token: Default::default(),
                                style: AttrStyle::Outer,
                                bracket_token: Default::default(),
                                meta: Meta::List(MetaList {
                                    path: Path {
                                        leading_colon: None,
                                        segments: create_punctuated(vec!["serde"]),
                                    },
                                    delimiter: MacroDelimiter::Paren(Paren::default()),
                                    tokens: token_stream,
                                }),
                            });
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

fn patch_generics(files: &mut BTreeMap<String, (File, BTreeMap<String, usize>)>) {
    let mut updated_files = BTreeMap::new();
    for key in files.keys().cloned() {
        updated_files.insert(key, false);
    }

    loop {
        // Go through all keys
        // Pop current key to be able to use the tree map whenever
        // Check for struct in local struct, if not found check in all of the map
        // Push the struct back inside

        let mut new_fixes = false;

        let keys: Vec<String> = files.keys().cloned().collect();
        for key in keys {
            let (mut ast, structs) = files.remove(&key).unwrap();

            let struct_idxs: Vec<usize> = structs.values().cloned().collect();

            // Iterate through each struct, get their generic total
            for idx in struct_idxs {
                let (left, arr) = ast.items.split_at_mut(idx);
                let (temp, right) = arr.split_at_mut(1);

                let curr_struct = as_struct(temp.get_mut(0).unwrap()).unwrap();
                let total_generics = curr_struct.generics.params.len();
                let mut new_total_generics = 0;

                // Iterate through each field and automatically update the fields and add to total field tally
                for field in as_named_fields(&mut curr_struct.fields)
                    .unwrap()
                    .named
                    .iter_mut()
                {
                    let name = field.ident.clone().unwrap().to_string();
                    let mut found_ty = None;
                    if let Some((field_ty, path)) = is_important(field) {
                        let ty = path.path.segments.last_mut().unwrap();

                        let ident_name = ty.ident.to_string();

                        fn push_generics(
                            ty_struct: &ItemStruct,
                            ty: &mut PathSegment,
                            mut new_total_generics: usize,
                        ) -> usize {
                            let mut args = Punctuated::new();

                            for _ in 0..ty_struct.generics.params.len() {
                                args.push(GenericArgument::Type(Type::Path(TypePath {
                                    qself: None,
                                    path: gen_generic(GENERICS[new_total_generics]),
                                })));
                                new_total_generics += 1;
                            }

                            ty.arguments =
                                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Default::default(),
                                    args,
                                    gt_token: Default::default(),
                                });

                            new_total_generics
                        }

                        // Generis state patch
                        if ident_name == "GenesisState"
                            && key == "ibc.core.types.v1.rs"
                            && name == "client_genesis"
                        {
                            let (key, s) = files.get_mut("ibc.core.client.v1.rs").unwrap();
                            let ty_struct = as_struct(
                                key.items.get_mut(*s.get("GenesisState").unwrap()).unwrap(),
                            )
                            .unwrap();
                            found_ty = Some(field_ty);
                            new_total_generics = push_generics(ty_struct, ty, new_total_generics);
                        } else if let Some(i) = structs.get(&ident_name) {
                            let ty_item = match i.cmp(&idx) {
                                Ordering::Less => left.get_mut(*i).unwrap(),
                                Ordering::Greater => right.get_mut(*i - idx - 1).unwrap(),
                                Ordering::Equal => continue,
                            };

                            let ty_struct = as_struct(ty_item).unwrap();
                            found_ty = Some(field_ty);
                            new_total_generics = push_generics(ty_struct, ty, new_total_generics);
                        } else if GENERICS.contains(&&*ident_name) {
                            ty.ident = Ident::new(GENERICS[new_total_generics], Span::call_site());
                            new_total_generics += 1;
                        } else if let Some((_, (other_ast, other_structs))) = files
                            .iter_mut()
                            .find(|(_, (_, s))| s.contains_key(&ident_name))
                        {
                            let ty_struct = as_struct(
                                other_ast
                                    .items
                                    .get_mut(*other_structs.get(&ident_name).unwrap())
                                    .unwrap(),
                            )
                            .unwrap();
                            found_ty = Some(field_ty);
                            new_total_generics = push_generics(ty_struct, ty, new_total_generics);
                        }
                    }

                    // Try to add field attrs
                    if let Some(found_ty) = found_ty {
                        let last = field.attrs.last().unwrap().clone();
                        if let Meta::List(meta_list) = &last.meta {
                            if meta_list.path.segments.last().unwrap().ident != "serde" {
                                // Set serialization function
                                let serde_path = match found_ty {
                                    FoundEnclosure::Option => "option",
                                    FoundEnclosure::Vec => "vec",
                                };

                                let mut token_stream = TokenStream::new();
                                token_stream
                                    .append(Ident::new("serialize_with", Span::call_site()));
                                token_stream.append(Punct::new('=', Spacing::Alone));
                                token_stream.append(Literal::string(&format!(
                                    "crate::any::{}::generic_serialize",
                                    serde_path
                                )));
                                token_stream.append(Punct::new(',', Spacing::Alone));
                                token_stream
                                    .append(Ident::new("deserialize_with", Span::call_site()));
                                token_stream.append(Punct::new('=', Spacing::Alone));
                                token_stream.append(Literal::string(&format!(
                                    "crate::any::{}::generic_deserialize",
                                    serde_path
                                )));

                                field.attrs.push(Attribute {
                                    pound_token: Default::default(),
                                    style: AttrStyle::Outer,
                                    bracket_token: Default::default(),
                                    meta: Meta::List(MetaList {
                                        path: Path {
                                            leading_colon: None,
                                            segments: create_punctuated(vec!["serde"]),
                                        },
                                        delimiter: MacroDelimiter::Paren(Paren::default()),
                                        tokens: token_stream,
                                    }),
                                });
                            }
                        }
                    }
                }

                if new_total_generics > 0 {
                    updated_files.insert(key.clone(), true);
                }

                if total_generics != new_total_generics {
                    curr_struct.generics.params.clear();

                    for gen in GENERICS[0..new_total_generics].iter() {
                        curr_struct
                            .generics
                            .params
                            .push(GenericParam::Type(gen_type_param(gen)));
                    }
                    new_fixes = true;
                }
            }

            files.insert(key, (ast, structs));
        }

        if !new_fixes {
            break;
        }
    }

    // Remove files that werent updated
    for (key, updated) in updated_files.iter() {
        if !updated {
            files.remove(key);
        }
    }
}

fn patch_impls(files: &mut BTreeMap<String, (File, BTreeMap<String, usize>)>) {
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
                let implemented = as_path(impl_item.self_ty.as_mut())
                    .map(|p| p.path.segments.last().unwrap().ident.to_string())
                    .unwrap_or("".to_string());

                let idx = match structs.get(&implemented) {
                    // If its not a struct then ignore
                    None => continue,
                    Some(idx) => *idx,
                };
                // Unwrap cause we know its a struct
                let struct_item = as_struct(left.get_mut(idx).unwrap()).unwrap();
                // Ignore structs with no generics
                if struct_item.generics.params.is_empty() {
                    continue;
                }

                impl_item.generics = struct_item.generics.clone();

                if let Some(impl_path) = as_path(impl_item.self_ty.as_mut()) {
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

fn save(out_dir: &str, files: &BTreeMap<String, (File, BTreeMap<String, usize>)>) {
    for (file, (data, _)) in files.iter() {
        // Patch the mod file
        let file_regex = Regex::new(r"(\.[[:alnum:]]+\.)rs").unwrap();
        let new_file = file_regex.replace(file, "${1}abstract.rs").to_string();

        patch_file(
            format!("{}/mod.rs", out_dir),
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
        fs::write(
            format!("{}/{}", out_dir, new_file),
            quote!(#data).to_string(),
        )
        .unwrap();
    }
}
