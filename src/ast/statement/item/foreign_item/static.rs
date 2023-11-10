use crate::{
    ast::{Attribute, Type, Visibility},
    tokens::{Colon, Ident, Mut, SemiColon, Static},
};

#[derive(Clone)]
pub struct ForeignItemStatic {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#static: Static,
    pub mutability: Option<Mut>,
    pub ident: Ident,
    pub colon: Colon,
    pub r#type: Box<Type>,
    pub semi_colon: SemiColon,
}
