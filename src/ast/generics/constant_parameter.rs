use crate::{
    ast::{Attribute, Expression, Type},
    tokens::{Colon, Const, Equals, Ident},
};

#[derive(Clone)]
pub struct ConstantParameter {
    pub attributes: Vec<Attribute>,
    pub r#const: Const,
    pub ident: Ident,
    pub colon: Colon,
    pub r#type: Type,
    pub equals: Option<Equals>,
    pub default: Option<Expression>,
}
