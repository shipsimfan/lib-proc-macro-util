use crate::ast::Expression;

mod operator;

mod parse;
mod to_static;
mod to_tokens;

pub use operator::CompoundAssignmentOperator;

/// A combined assigned and arithmetic or logical operation
#[derive(Debug, Clone)]
pub struct CompoundAssignmentExpression<'a> {
    /// The left side of the operation
    pub left: Box<Expression<'a>>,

    /// The operator to use
    pub operator: CompoundAssignmentOperator,

    /// The right side of the operation
    pub right: Box<Expression<'a>>,
}
