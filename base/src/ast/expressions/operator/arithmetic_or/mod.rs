use crate::ast::Expression;

mod operator;

mod parse;
mod to_static;
mod to_tokens;

pub use operator::ArithmeticOrLogicalOperator;

/// A comparison between two expressions
#[derive(Debug, Clone)]
pub struct ArithmeticOrLogicalExpression<'a> {
    /// The left side of the comparison
    pub left: Box<Expression<'a>>,

    /// The operator to compare with
    pub operator: ArithmeticOrLogicalOperator,

    /// The right side of the comparison
    pub right: Box<Expression<'a>>,
}
