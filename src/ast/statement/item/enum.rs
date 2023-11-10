use crate::{
    ast::{Attribute, Generics, Punctuated, Variant, Visibility},
    tokens::{Brace, Comma, Enum, Ident},
};

#[derive(Clone)]
pub struct ItemEnum {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#enum: Enum,
    pub ident: Ident,
    pub generics: Generics,
    pub brace: Brace,
    pub variants: Punctuated<Variant, Comma>,
}
