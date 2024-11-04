use crate::parser::utils::common::{create_punctuated, gen_generic};
use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TraitBound,
    TraitBoundModifier, Type, TypeParam, TypeParamBound, TypePath,
};

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
}

pub fn gen_type_param(name: &str) -> TypeParam {
    let mut type_param = gen_unnamed_param(name);
    type_param
        .bounds
        .push(trait_param_bound(vec!["prost", "Name"]));
    type_param
}

pub fn gen_any(name: &str) -> Path {
    let mut paths = create_punctuated(vec!["crate", "any"]);

    let mut punctuation = Punctuated::new();
    punctuation.push(GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: gen_generic(name),
    })));

    paths.push(PathSegment {
        ident: Ident::new("Any", Span::call_site()),
        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: punctuation,
            gt_token: Default::default(),
        }),
    });

    Path {
        leading_colon: None,
        segments: paths,
    }
}
