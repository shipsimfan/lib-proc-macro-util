use crate::{
    ast::{Attribute, Expression, Generics, Type, Visibility},
    tokens::{Colon, Const, Default, Equals, Ident, SemiColon},
};

#[derive(Clone)]
pub struct ImplItemConst {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub default: Option<Default>,
    pub r#const: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub r#type: Type,
    pub equals: Equals,
    pub expression: Expression,
    pub semi_colon: SemiColon,
}
