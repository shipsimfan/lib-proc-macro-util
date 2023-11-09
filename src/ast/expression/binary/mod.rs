use crate::ast::{Attribute, Expression};

mod operator;

pub use operator::BinaryOperator;

#[derive(Clone)]
pub struct ExpressionBinary {
    pub attributes: Vec<Attribute>,
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}
