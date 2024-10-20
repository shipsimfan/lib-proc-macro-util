use crate::{tokens::Identifier, Token};

mod parse;
mod to_tokens;

/// An identifier which can be anonymous
#[derive(Debug, Clone)]
pub enum MaybeIdentifier<'a> {
    /// The identifier is named
    Identifier(&'a Identifier),

    /// The identifier is anonymous
    Underscore(Token![_]),
}
