use crate::{tokens::Identifier, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// An identifier which can be anonymous
#[derive(Debug, Clone)]
pub enum MaybeIdentifier<'a> {
    /// The identifier is named
    Identifier(Cow<'a, Identifier>),

    /// The identifier is anonymous
    Underscore(Token![_]),
}
