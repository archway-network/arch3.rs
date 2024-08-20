use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use syn::{
    Field,
    Fields,
    FieldsNamed,
    GenericArgument,
    Ident,
    Item,
    ItemStruct,
    Path,
    PathArguments,
    PathSegment,
    TraitBound,
    TraitBoundModifier,
    Type,
    TypeParam,
    TypeParamBound,
    TypePath,
};

pub fn item_as_struct(item: &mut Item) -> Option<&mut ItemStruct> {
    match item {
        Item::Struct(s) => Some(s),
        _ => None,
    }
}

pub fn type_as_path(item: &mut Type) -> Option<&mut TypePath> {
    match item {
        Type::Path(ret) => Some(ret),
        _ => None,
    }
}

pub fn fields_as_named(fields: &mut Fields) -> Option<&mut FieldsNamed> {
    match fields {
        Fields::Named(ret) => Some(ret),
        _ => None,
    }
}

pub fn create_punctuated(path: Vec<&str>) -> Punctuated<PathSegment, PathSep> {
    let mut ret = Punctuated::new();
    for p in path {
        ret.push(PathSegment {
            ident: Ident::new(p, Span::call_site()),
            arguments: PathArguments::None,
        });
    }
    ret
}

pub fn gen_generic(name: &str) -> Path {
    Path {
        leading_colon: None,
        segments: create_punctuated(vec![name]),
    }
}
