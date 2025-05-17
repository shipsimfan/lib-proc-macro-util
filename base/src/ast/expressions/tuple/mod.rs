use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A tuple made up of multiple expressions
#[derive(Debug, Clone)]
pub struct TupleExpression<'a> {
    /// The first element in the tuple
    pub first: Option<(Box<Expression<'a>>, Token![,])>,

    /// The remaining elements with separators
    pub remaining: Vec<(Expression<'a>, Token![,])>,

    /// The last element
    pub last: Option<Box<Expression<'a>>>,
}
