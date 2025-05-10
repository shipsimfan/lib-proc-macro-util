use crate::ast::Expression;

mod operator;

mod parse;
mod to_static;
mod to_tokens;

pub use operator::ComparisonOperator;

/// A comparison between two expressions
#[derive(Debug, Clone)]
pub struct ComparisonExpression<'a> {
    /// The left side of the comparison
    pub left: Box<Expression<'a>>,

    /// The operator to compare with
    pub operator: ComparisonOperator,

    /// The right side of the comparison
    pub right: Box<Expression<'a>>,
}
