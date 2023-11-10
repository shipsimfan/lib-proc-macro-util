use crate::{
    ast::{Attribute, Expression, Generics, Type, Visibility},
    tokens::{Colon, Const, Equals, Ident, SemiColon},
};

#[derive(Clone)]
pub struct ItemConst {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#const: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub r#type: Box<Type>,
    pub equals: Equals,
    pub expression: Box<Expression>,
    pub semi_colon: SemiColon,
}
