use crate::ast::expressions::LiteralExpression;

mod from;
mod new;
mod parse;
mod to_tokens;

/// A specific kind of expression
#[derive(Debug, Clone)]
pub enum ExpressionKind<'a> {
    /// An expression made up of a literal value
    Literal(LiteralExpression<'a>),
}
