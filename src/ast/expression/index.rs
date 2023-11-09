use crate::{
    ast::{Attribute, Expression},
    tokens::Bracket,
};

#[derive(Clone)]
pub struct ExpressionIndex {
    pub attributes: Vec<Attribute>,
    pub expression: Box<Expression>,
    pub bracket: Bracket,
    pub index: Box<Expression>,
}
