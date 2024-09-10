use crate::{tokens::TokenTree, Error, Parser, Span};

impl<'a> Parser<'a> {
    /// Get the span of the next element
    ///
    /// ## Return Value
    /// Returns the [`Span`] of the next element or `Span:call_site()` if there is none.
    pub fn span(&self) -> Span {
        match self.stream.get(self.index) {
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
    pub fn error<T: std::fmt::Display>(&self, message: T) -> Error {
        Error::new_at(message, self.span())
    }
}
