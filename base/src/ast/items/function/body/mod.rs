use crate::{ast::expressions::BlockExpression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A body of a function
#[derive(Debug, Clone)]
pub enum FunctionBody<'a> {
    /// The body is defined here
    Expression(BlockExpression<'a>),

    /// The body is not defined here
    Semi(Token![;]),
}
