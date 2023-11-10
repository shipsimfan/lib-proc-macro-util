use crate::{
    ast::{Attribute, FieldsNamed, Generics, Visibility},
    tokens::{Ident, Union},
};

#[derive(Clone)]
pub struct ItemUnion {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#union: Union,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}
