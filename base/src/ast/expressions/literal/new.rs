use crate::{ast::expressions::LiteralExpression, tokens::Literal, Token};
use std::borrow::Cow;

impl<'a> LiteralExpression<'a> {
    /// Creates a new [`LiteralExpression`] from `literal`
    pub fn new<T: Into<Cow<'a, Literal>>>(literal: T) -> Self {
        LiteralExpression::Literal(literal.into())
    }

    /// Creates a new [`LiteralExpression::True`]
    pub fn new_true() -> Self {
        LiteralExpression::True(Token![true]())
    }

    /// Creates a new [`LiteralExpression::False`]
    pub fn new_false() -> Self {
        LiteralExpression::False(Token![false]())
    }

    /// Creates a new [`LiteralExpression`] based on `b`
    pub fn new_bool(b: bool) -> Self {
        if b {
            LiteralExpression::new_true()
        } else {
            LiteralExpression::new_false()
        }
    }
}
