use crate::{
    ast::{Attribute, Expression},
    tokens::Equals,
};

#[derive(Clone)]
pub struct ExpressionAssign {
    pub attributes: Vec<Attribute>,
    pub left: Box<Expression>,
    pub equals: Equals,
    pub right: Box<Expression>,
}
