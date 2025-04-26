use crate::{ast::Expression, tokens::Identifier, Token};
use std::borrow::Cow;

mod to_static;
mod to_tokens;

/// A accessor of a field in a struct or union
#[derive(Debug, Clone)]
pub struct FieldExpression<'a> {
    /// The expression being accessed
    pub expression: Box<Expression<'a>>,

    /// The dot indicating the access
    pub dot: Token![.],

    /// The field being accessed
    pub identifier: Cow<'a, Identifier>,
}
