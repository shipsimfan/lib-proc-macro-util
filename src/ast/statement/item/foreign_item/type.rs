use crate::{
    ast::{Attribute, Generics, Type, Visibility},
    tokens::{Ident, SemiColon},
};

#[derive(Clone)]
pub struct ForeignItemType {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#type: Type,
    pub ident: Ident,
    pub generics: Generics,
    pub semi_colon: SemiColon,
}
