use crate::{tokens::Identifier, Token};

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

/// A single segment of a [`SimplePath`](super::SimplePath)
#[derive(Debug, Clone)]
pub enum SimplePathSegment<'a> {
    /// A borrowed identifier
    Identifier(&'a Identifier),

    /// An owned identifier
    OwnedIdentifier(Identifier),

    /// A reference to the local crate used in `macro_rules!`
    DollarCrate(Token![$], Token![crate]),
}
