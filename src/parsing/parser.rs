use crate::{
    tokens::{Group, Identifier, Literal, Punctuation, TokenTree},
    Parse, Result, TokenBuffer,
};

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

    /// Gets the next token in the stream
    ///
    /// ## Return Value
    /// Returns the next [`TokenTree`] in the stream, if it exists
    pub fn next(&mut self) -> Option<TokenTree<'a>> {
        let ret = self.current()?;
        self.advance();
        Some(ret)
    }

    /// Parses a [`Group`] from the stream
    ///
    /// ## Return Value
    /// Returns the [`Group`] if it is the next token in the stream
    ///
    /// ## Remarks
    /// This function advances the parser only if it finds a [`Group`]
    pub fn group(&mut self) -> Option<Group<'a>> {
        match self.current()? {
            TokenTree::Group(group) => {
                self.advance();
                Some(group)
            }
            _ => None,
        }
    }

    /// Parses a [`Identifier`] from the stream
    ///
    /// ## Return Value
    /// Returns the [`Identifier`] if it is the next token in the stream
    ///
    /// ## Remarks
    /// This function advances the parser only if it finds a [`Identifier`]
    pub fn identifier(&mut self) -> Option<Identifier> {
        match self.current()? {
            TokenTree::Identifier(identifier) => {
                self.advance();
                Some(identifier)
            }
            _ => None,
        }
    }

    /// Parses a [`Literal`] from the stream
    ///
    /// ## Return Value
    /// Returns the [`Literal`] if it is the next token in the stream
    ///
    /// ## Remarks
    /// This function advances the parser only if it finds a [`Literal`]
    pub fn literal(&mut self) -> Option<Literal> {
        match self.current()? {
            TokenTree::Literal(literal) => {
                self.advance();
                Some(literal)
            }
            _ => None,
        }
    }

    /// Parses a [`Punctuation`] from the stream
    ///
    /// ## Return Value
    /// Returns the [`Punctuation`] if it is the next token in the stream
    ///
    /// ## Remarks
    /// This function advances the parser only if it finds a [`Punctuation`]
    pub fn punctuation(&mut self) -> Option<Punctuation> {
        match self.current()? {
            TokenTree::Punctuation(punctuation) => {
                self.advance();
                Some(punctuation)
            }
            _ => None,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
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
