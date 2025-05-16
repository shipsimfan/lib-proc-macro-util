use crate::ast::Expression;

mod operator;

mod parse;
mod to_static;
mod to_tokens;

pub use operator::NegationOperator;

/// A negation of another expression
#[derive(Debug, Clone)]
pub struct NegationExpression<'a> {
    /// The operator being applied
    pub operator: NegationOperator,

    /// The expression being borrowed
    pub expression: Box<Expression<'a>>,
}
