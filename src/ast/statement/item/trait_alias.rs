use crate::{
    ast::{Attribute, Generics, Punctuated, TypeParameterBound, Visibility},
    tokens::{Equals, Ident, Plus, SemiColon, Trait},
};

#[derive(Clone)]
pub struct ItemTraitAlias {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#trait: Trait,
    pub ident: Ident,
    pub generics: Generics,
    pub equals: Equals,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
    pub semi_colon: SemiColon,
}
