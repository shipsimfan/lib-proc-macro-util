use crate::{ast::PatternNoTopAlt, tokens::Identifier, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// Identifier patterns bind the value they match to a variable in the value namespace.
#[derive(Debug, Clone)]
pub struct IdentifierPattern<'a> {
    /// Is this a reference?
    pub r#ref: Option<Token![ref]>,

    /// Is this mutable?
    pub r#mut: Option<Token![mut]>,

    /// The identifier itself
    pub identifier: Cow<'a, Identifier>,

    /// A pattern to restrict what is matched
    pub pattern: Option<(Token![@], Box<PatternNoTopAlt<'a>>)>,
}
