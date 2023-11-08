use crate::{
    ast::{AngleBracketGenerics, Punctuated, TypeParameterBound},
    tokens::{Colon, Ident, Plus},
};

#[derive(Clone)]
pub struct Constraint {
    pub ident: Ident,
    pub generics: Option<AngleBracketGenerics>,
    pub colon: Colon,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
}
