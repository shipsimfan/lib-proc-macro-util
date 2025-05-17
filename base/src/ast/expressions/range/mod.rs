use crate::ast::Expression;

mod operator;

mod to_static;
mod to_tokens;

pub use operator::RangeOperator;

/// A range between two expressions
#[derive(Debug, Clone)]
pub struct RangeExpression<'a> {
    /// The lower bound of the range
    pub lower: Option<Box<Expression<'a>>>,

    /// The operator used in the range
    pub operator: RangeOperator,

    /// The upper bound of the range
    pub upper: Option<Box<Expression<'a>>>,
}
