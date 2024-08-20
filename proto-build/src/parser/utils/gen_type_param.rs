use crate::parser::utils::common::create_punctuated;
use proc_macro2::{Ident, Span};
use syn::{Path, TraitBound, TraitBoundModifier, TypeParam, TypeParamBound};

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

pub fn gen_type_param(name: &str) -> TypeParam {
    let mut type_param = gen_unnamed_param(name);
    type_param
        .bounds
        .push(trait_param_bound(vec!["prost", "Name"]));
    type_param
}
