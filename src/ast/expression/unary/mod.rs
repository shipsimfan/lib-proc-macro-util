use crate::ast::{Attribute, Expression};

mod operator;

pub use operator::UnaryOperator;

#[derive(Clone)]
pub struct ExpressionUnary {
    pub attributes: Vec<Attribute>,
    pub operator: UnaryOperator,
    pub expression: Box<Expression>,
}
