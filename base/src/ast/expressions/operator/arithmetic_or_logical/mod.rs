use crate::ast::Expression;

mod operator;

mod parse;
mod to_static;
mod to_tokens;

pub use operator::ArithmeticOrLogicalOperator;

/// A arithmetic or logical operation between two expressions
#[derive(Debug, Clone)]
pub struct ArithmeticOrLogicalExpression<'a> {
    /// The left side of the operation
    pub left: Box<Expression<'a>>,

    /// The operator to use
    pub operator: ArithmeticOrLogicalOperator,

    /// The right side of the operation
    pub right: Box<Expression<'a>>,
}
