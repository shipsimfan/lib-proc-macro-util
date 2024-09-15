use crate::{tokens::Literal, Token};

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

/// An expression made up of a literal value
#[derive(Debug, Clone)]
pub enum LiteralExpression<'a> {
    /// A normal borrowed literal
    Literal(&'a Literal),

    /// A normal owned literal
    OwnedLiteral(Literal),

    /// Boolean value `true`
    True(Token![true]),

    /// Boolean value `false`
    False(Token![false]),
}
