use crate::{tokens::Identifier, Token};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// A label identifying a loop
#[derive(Debug, Clone)]
pub struct LoopLabel<'a> {
    /// The quote introducing the label
    pub quote: Token!['_],

    /// The name of the label
    pub name: Cow<'a, Identifier>,

    /// The colon separating the loop
    pub colon: Token![:],
}
