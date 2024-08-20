use std::cmp::Ordering;
use std::collections::BTreeMap;
use proc_macro2::{Ident, Literal, Punct, Spacing, Span, TokenStream};
use quote::TokenStreamExt;
use syn::{AngleBracketedGenericArguments, Attribute, AttrStyle, File, GenericArgument, GenericParam, ItemStruct, MacroDelimiter, Meta, MetaList, Path, PathArguments, PathSegment, Type, TypePath};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use crate::parser::consts::GENERICS;
use crate::parser::utils::is_important::{FoundEnclosure, is_important};
use crate::parser::utils::common::{fields_as_named, item_as_struct, create_punctuated, gen_generic};
use crate::parser::utils::gen_type_param::gen_type_param;

pub fn patch_generics(files: &mut BTreeMap<String, (File, BTreeMap<String, usize>)>) {
    let mut updated_files = BTreeMap::new();
    for key in files.keys().cloned() {
        updated_files.insert(key, false);
    }

    loop {
        // Go through all keys
        // Pop current key to be able to use the tree map whenever
        // Check for struct in local struct, if not found check in `all` of the map
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

                let curr_struct = item_as_struct(temp.get_mut(0).unwrap()).unwrap();
                let total_generics = curr_struct.generics.params.len();
                let mut new_total_generics = 0;

                // Iterate through each field and automatically update the fields and add to total field tally
                for field in fields_as_named(&mut curr_struct.fields)
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
                            let ty_struct = item_as_struct(
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

                            let ty_struct = item_as_struct(ty_item).unwrap();
                            found_ty = Some(field_ty);
                            new_total_generics = push_generics(ty_struct, ty, new_total_generics);
                        } else if GENERICS.contains(&&*ident_name) {
                            ty.ident = Ident::new(GENERICS[new_total_generics], Span::call_site());
                            new_total_generics += 1;
                        } else if let Some((_, (other_ast, other_structs))) = files
                            .iter_mut()
                            .find(|(_, (_, s))| s.contains_key(&ident_name))
                        {
                            let ty_struct = item_as_struct(
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
