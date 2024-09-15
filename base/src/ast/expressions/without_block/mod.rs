use crate::ast::expressions::LiteralExpression;

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

/// An expression that does not have a block
#[derive(Debug, Clone)]
pub enum ExpressionWithoutBlock<'a> {
    /// An expression made up of a literal value
    Literal(LiteralExpression<'a>),
}
