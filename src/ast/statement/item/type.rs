use crate::{
    ast::{Attribute, Generics, Type, Visibility},
    tokens::{Equals, Ident, SemiColon},
};

#[derive(Clone)]
pub struct ItemType {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub type_token: crate::tokens::Type,
    pub ident: Ident,
    pub generics: Generics,
    pub equals: Equals,
    pub r#type: Box<Type>,
    pub semi_colon: SemiColon,
}
