use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// The question mark operator (?) unwraps valid values or returns erroneous values, propagating
/// them to the calling function.
#[derive(Debug, Clone)]
pub struct ErrorPropagationExpression<'a> {
    /// The expression to be evaluated for errors
    pub expression: Box<Expression<'a>>,

    /// The question mark identifying this expression
    pub question: Token![?],
}
