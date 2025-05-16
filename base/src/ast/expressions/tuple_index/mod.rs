use crate::{ast::Expression, tokens::Literal, Token};
use std::borrow::Cow;

mod to_static;
mod to_tokens;

/// Accesses an element from a tuple
#[derive(Debug, Clone)]
pub struct TupleIndexExpression<'a> {
    /// The expression to index
    pub expression: Box<Expression<'a>>,

    /// The dot indicating this may be a tuple index
    pub dot: Token![.],

    /// The index into the tuple to access
    pub index: Cow<'a, Literal>,
}
