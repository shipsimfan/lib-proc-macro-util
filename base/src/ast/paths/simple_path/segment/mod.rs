use crate::{tokens::Identifier, Token};
use std::borrow::Cow;

mod display;
mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A single segment of a [`SimplePath`](super::SimplePath)
#[derive(Debug, Clone)]
pub enum SimplePathSegment<'a> {
    /// A borrowed identifier
    Identifier(Cow<'a, Identifier>),

    /// A reference to the local crate
    Crate(Token![crate]),

    /// A reference to the local item
    _Self(Token![self]),

    /// A reference to super
    Super(Token![super]),

    /// A reference to the local crate used in `macro_rules!`
    DollarCrate(Token![$], Token![crate]),
}
