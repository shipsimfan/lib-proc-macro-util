use crate::{
    ast::{Attribute, Expression, Type, Visibility},
    tokens::{Colon, Equals, Ident, Mut, SemiColon, Static},
};

#[derive(Clone)]
pub struct ItemStatic {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#static: Static,
    pub r#mut: Option<Mut>,
    pub ident: Ident,
    pub colon: Colon,
    pub r#type: Box<Type>,
    pub equals: Equals,
    pub expression: Box<Expression>,
    pub semi_colon: SemiColon,
}
