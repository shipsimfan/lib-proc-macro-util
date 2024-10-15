use crate::{tokens::TokenTree, Parse, Parser, Result};

impl<'a> Parser<'a> {
    /// Checks the next element in the stream without advancing it
    ///
    /// ## Return Value
    /// Returns `true` if the next value in stream is of type `T`
    pub fn peek<T: Parse<'a>>(&self) -> bool {
        let mut parser = self.clone();
        parser.parse::<T>().is_ok()
    }

    /// Checks if an element `n` tokens ahead matches that target element without advancing the
    /// stream
    pub fn peek_n<T: Parse<'a>>(&self, n: usize) -> bool {
        let mut parser = self.clone();
        parser.skip(n);
        parser.parse::<T>().is_ok()
    }

    /// Skips `n` tokens from the stream
    pub fn skip(&mut self, n: usize) {
        self.index += n;
    }

    /// Gets the next token in the stream
    ///
    /// ## Return Value
    /// Returns the next [`TokenTree`] in the stream, if it exists
    pub fn next(&mut self) -> Option<&'a TokenTree> {
        let ret = self.stream.get(self.index)?;
        self.index += 1;
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

    /// Parse an object from this parser's stream
    ///
    /// ## Return Value
    /// Returns the newly created object on success.
    pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
        T::parse(self)
    }

    /// Parses an object from the stream, only advancing if the parse succeeds
    pub fn step_parse<T: Parse<'a>>(&mut self) -> Result<T> {
        self.step(Parser::parse)
    }

    /// Collects the remaining tokens in the stream
    pub fn collect(&mut self) -> &'a [TokenTree] {
        let ret = if self.index > self.stream.len() {
            &[]
        } else {
            &self.stream[self.index..]
        };
        self.index = self.stream.len();
        ret
    }
}
