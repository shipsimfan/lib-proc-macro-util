use crate::ast::Expression;

mod parse;
mod to_static;
mod to_tokens;

/// A grouped expression wraps a single expression, evaluating to that expression
#[derive(Debug, Clone)]
pub struct GroupedExpression<'a> {
    /// The contained expression
    pub expression: Box<Expression<'a>>,
}
