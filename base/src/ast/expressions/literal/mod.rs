use crate::{tokens::Literal, Token};
use std::borrow::Cow;

mod display;
mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// An expression made up of a literal value
#[derive(Debug, Clone)]
pub enum LiteralExpression<'a> {
    /// A normal borrowed literal
    Literal(Cow<'a, Literal>),

    /// Boolean value `true`
    True(Token![true]),

    /// Boolean value `false`
    False(Token![false]),
}
