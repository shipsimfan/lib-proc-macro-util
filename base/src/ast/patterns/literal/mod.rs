use crate::{tokens::Literal, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// Literal patterns match exactly the same value as what is created by the literal.
#[derive(Debug, Clone)]
pub enum LiteralPattern<'a> {
    /// The literal is actually a literal
    Literal(Option<Token![-]>, Cow<'a, Literal>),

    /// The literal is the true keyword
    True(Token![true]),

    /// The literal is the false keyword
    False(Token![false]),
}
