use crate::{tokens::Identifier, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// A name of an external crate
#[derive(Debug, Clone)]
pub enum CrateRef<'a> {
    /// The name of a crate
    Identifier(Cow<'a, Identifier>),

    /// This crate
    _Self(Token![self]),
}
