use crate::{
    tokens::{Group, Identifier, Literal, Punctuation, TokenTree},
    Error, Parse, Result, ToTokens, TokenBuffer,
};
use proc_macro::Span;
use std::fmt::Display;

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
    pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
        T::parse(self)
    }

    /// Is this stream empty?
    ///
    /// ## Return Value
    /// Returns true if there are no more tokens in the stream
    pub fn empty(&self) -> bool {
        self.index >= self.buffer.len()
    }

    /// Checks the next element in the stream without advancing it
    ///
    /// ## Return Value
    /// Returns `true` if the next value in stream is of type `T`
    pub fn peek<T: Parse<'a>>(&self) -> bool {
        let mut parser = self.clone();
        parser.parse::<T>().is_ok()
    }

    /// Get the span of the next element
    ///
    /// ## Return Value
    /// Returns the [`Span`] of the next element or `Span:call_site()` if there is none.
    pub fn span(&self) -> Span {
        match self.current() {
            Some(current) => match current {
                TokenTree::Group(group) => group.span,
                TokenTree::Identifier(identifier) => identifier.span(),
                TokenTree::Literal(literal) => literal.span(),
                TokenTree::Punctuation(punctuation) => punctuation.span(),
            },
            None => Span::call_site(),
        }
    }

    /// Creates an error using the [`Span`] of the next element
    ///
    /// ## Parameters
    ///  * `message` - The message which will be displayed
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn error<T: Display>(&self, message: T) -> Error {
        Error::new_at(message, self.span())
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

    /// Runs a parsing function only advancing if the parse succeeds
    ///
    /// ## Parameters
    ///  * `f` - The parsing function to be run
    ///
    /// ## Return Value
    /// Returns the value return by `f` on success
    pub fn step<T, F: FnOnce(&mut Parser<'a>) -> Result<T>>(&mut self, f: F) -> Result<T> {
        let mut parser = self.clone();

        f(&mut parser).map(|value| {
            *self = parser;
            value
        })
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

impl<'a> Into<TokenBuffer> for Parser<'a> {
    fn into(mut self) -> TokenBuffer {
        let mut token_buffer = TokenBuffer::new();
        while let Some(token) = self.next() {
            token_buffer.push(token.into());
        }
        token_buffer
    }
}

impl<'a> ToTokens for Parser<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        self.buffer.to_tokens(generator);
    }
}
