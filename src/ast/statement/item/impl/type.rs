use crate::{
    ast::{Attribute, Generics, Type, Visibility},
    tokens::{Default, Equals, Ident, SemiColon},
};

#[derive(Clone)]
pub struct ImplItemType {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub default: Option<Default>,
    pub type_token: crate::tokens::Type,
    pub ident: Ident,
    pub generics: Generics,
    pub equals: Equals,
    pub r#type: Type,
    pub semi_colon: SemiColon,
}
