use crate::tokens::TokenTree;

mod from;
mod next;
mod span;
mod tree;

/// A source of tokens for parsing
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    /// The buffer holding the tokens
    stream: &'a [TokenTree],

    /// The current location of this parser in the stream
    index: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] over `stream`
    pub fn new(stream: &'a [TokenTree]) -> Self {
        Parser { stream, index: 0 }
    }

    /// Is this stream empty?
    ///
    /// ## Return Value
    /// Returns true if there are no more tokens in the stream
    pub fn empty(&self) -> bool {
        self.index >= self.stream.len()
    }
}
