use crate::{
    ast::{Attribute, Expression},
    tokens::{Ampersand, Mut},
};

#[derive(Clone)]
pub struct ExpressionReference {
    pub attributes: Vec<Attribute>,
    pub ampersand: Ampersand,
    pub mutability: Option<Mut>,
    pub expression: Box<Expression>,
}
