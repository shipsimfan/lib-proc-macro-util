use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// Ends the running of a function, optionally returning a value
#[derive(Debug, Clone)]
pub struct ReturnExpression<'a> {
    /// The return keyword identifying this expression
    pub r#return: Token![break],

    /// An expression to return with
    pub expression: Option<Box<Expression<'a>>>,
}
