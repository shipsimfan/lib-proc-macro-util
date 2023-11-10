use crate::{
    ast::{Attribute, Generics, Punctuated, Type, TypeParameterBound},
    tokens::{Colon, Equals, Ident, Plus, SemiColon},
};

#[derive(Clone)]
pub struct TraitItemType {
    pub attributes: Vec<Attribute>,
    pub r#type: crate::tokens::Type,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Colon>,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
    pub default: Option<(Equals, Type)>,
    pub semi_colon: SemiColon,
}
