use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A dereference of another expression
#[derive(Debug, Clone)]
pub struct DereferenceExpression<'a> {
    /// The leading asterick indicating this is a dereference
    pub asterick: Token![*],

    /// The expression being dereferenced
    pub expression: Box<Expression<'a>>,
}
