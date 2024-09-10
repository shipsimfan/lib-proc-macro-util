use crate::{
    tokens::{Group, Identifier, Literal, Punctuation, TokenTree},
    Parser,
};

impl<'a> Parser<'a> {
    /// Parses a [`Group`] from the stream
    ///
    /// ## Return Value
    /// Returns the [`Group`] if it is the next token in the stream
    ///
    /// ## Remarks
    /// This function advances the parser only if it finds a [`Group`]
    pub fn group(&mut self) -> Option<&'a Group> {
        match self.stream.get(self.index)? {
            TokenTree::Group(group) => {
                self.index += 1;
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
    pub fn identifier(&mut self) -> Option<&'a Identifier> {
        match self.stream.get(self.index)? {
            TokenTree::Identifier(identifier) => {
                self.index += 1;
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
    pub fn literal(&mut self) -> Option<&'a Literal> {
        match self.stream.get(self.index)? {
            TokenTree::Literal(literal) => {
                self.index += 1;
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
    pub fn punctuation(&mut self) -> Option<&'a Punctuation> {
        match self.stream.get(self.index)? {
            TokenTree::Punctuation(punctuation) => {
                self.index += 1;
                Some(punctuation)
            }
            _ => None,
        }
    }
}
