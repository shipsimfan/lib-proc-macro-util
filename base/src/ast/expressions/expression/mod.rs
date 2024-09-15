use crate::ast::expressions::LiteralExpression;

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

/// A construct that evaluates to a value, and it can be used to perform computations, manipulate
/// data, or control the flow of a program
#[derive(Debug, Clone)]
pub enum Expression<'a> {
    /// An expression made up of a literal value
    Literal(LiteralExpression<'a>),
}
