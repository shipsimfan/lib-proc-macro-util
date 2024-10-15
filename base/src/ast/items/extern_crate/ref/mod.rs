use crate::{tokens::Identifier, Token};

mod parse;
mod to_tokens;

/// A name of an external crate
#[derive(Debug, Clone)]
pub enum CrateRef<'a> {
    /// The name of a crate
    Identifier(&'a Identifier),

    /// This crate
    _Self(Token![self]),
}
