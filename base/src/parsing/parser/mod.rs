use crate::{tokens::TokenTree, Span};

mod from;
mod new;
mod next;
mod span;

/// A source of tokens for parsing
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    /// The buffer holding the tokens
    stream: &'a [TokenTree],

    /// The current location of this parser in the stream
    index: usize,

    /// The span to produce for end errors
    span: Span,
}

impl<'a> Parser<'a> {
    /// Is this stream empty?
    ///
    /// ## Return Value
    /// Returns true if there are no more tokens in the stream
    pub fn empty(&self) -> bool {
        self.index >= self.stream.len()
    }
}
