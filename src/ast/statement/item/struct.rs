use crate::{
    ast::{Attribute, Fields, Generics, Visibility},
    tokens::{Ident, SemiColon, Struct},
};

#[derive(Clone)]
pub struct ItemStruct {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#struct: Struct,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
    pub semi_colon: SemiColon,
}
