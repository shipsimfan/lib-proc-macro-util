use crate::{tokens::TokenTree, Parse, Result, TokenBuffer};

/// A source of tokens for parsing
#[derive(Clone)]
pub struct Parser<'a> {
    /// The buffer holding the tokens
    buffer: &'a TokenBuffer,

    /// The current location of this parser in the stream
    index: usize,
}

impl<'a> Parser<'a> {
    /// Parse an object from this parser's stream
    ///
    /// ## Return Value
    /// Returns the newly created object on success.
    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    #[allow(unused)]
    fn current(&self) -> Option<TokenTree<'a>> {
        self.buffer.get(self.index)
    }
}

impl<'a> From<&'a TokenBuffer> for Parser<'a> {
    fn from(buffer: &'a TokenBuffer) -> Self {
        Parser { buffer, index: 0 }
    }
}
