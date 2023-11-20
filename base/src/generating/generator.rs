use crate::{
    parsing::TokenBuffer,
    tokens::{Identifier, Literal, OwnedGroup, OwnedTokenTree, Punctuation, TokenTree},
    ToTokens,
};
use proc_macro::{Delimiter, Span};

/// An object for generating tokens
pub struct Generator<'a> {
    tokens: &'a mut TokenBuffer,
}

impl<'a> Generator<'a> {
    /// Creates a new [`Generator`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Generator`]
    pub(crate) fn new(tokens: &'a mut TokenBuffer) -> Self {
        Generator { tokens }
    }

    /// Generates the tokens for the requested value
    ///
    /// ## Parameters
    ///  * `value` - The value to generate tokens for
    pub fn generate<T: ToTokens>(&mut self, value: &T) {
        value.to_tokens(self)
    }

    /// Appends the token tree into the stream
    ///
    /// ## Parameters
    ///  * `token` - The token tree to append
    pub fn push(&mut self, token: &TokenTree) {
        self.tokens.push(token.clone().into())
    }

    /// Appends an [`Identifier`] to the stream
    ///
    /// ## Parameters
    ///  * `identifier` - The [`Identifier`] to append
    pub fn identifier(&mut self, identifier: Identifier) {
        self.tokens.push(OwnedTokenTree::Identifier(identifier))
    }

    /// Appends an [`Identifier`] from a string to the stream
    ///
    /// ## Parameters
    ///  * `identifier` - The string to become the [`Identifier`] to append
    pub fn identifier_string(&mut self, identifier: &str) {
        self.identifier_string_at(identifier, Span::call_site())
    }

    /// Appends an [`Identifier`] from a string to the stream
    ///
    /// ## Parameters
    ///  * `identifier` - The string to become the [`Identifier`] to append
    ///  * `span` - The [`Span`] for the new [`Identifier`]
    pub fn identifier_string_at(&mut self, identifier: &str, span: Span) {
        self.identifier(Identifier::new(identifier, span))
    }

    /// Appends a [`Punctuation`] to the stream
    ///
    /// ## Parameters
    ///  * `punctuation` - The [`Punctuation`] to append
    pub fn punctuation(&mut self, punctuation: Punctuation) {
        self.tokens.push(OwnedTokenTree::Punctuation(punctuation))
    }

    /// Appends a [`Literal`] to the stream
    ///
    /// ## Parameters
    ///  * `literal` - The [`Literal`] to append to the stream
    pub fn literal(&mut self, literal: Literal) {
        self.tokens.push(OwnedTokenTree::Literal(literal))
    }

    /// Appends an [`Literal`] from a string to the stream
    ///
    /// ## Parameters
    ///  * `literal` - The string to become the [`Literal`] to append
    pub fn literal_string(&mut self, literal: &str) {
        self.literal_string_at(literal, Span::call_site())
    }

    /// Appends an [`Literal`] from a string to the stream
    ///
    /// ## Parameters
    ///  * `literal` - The string to become the [`Literal`] to append
    ///  * `span` - The [`Span`] for the new [`Literal`]
    pub fn literal_string_at(&mut self, literal: &str, span: Span) {
        self.literal(Literal::new_string(literal, span))
    }

    /// Appends a group to the stream
    ///
    /// ## Parameters
    ///  * `delimiter` - The [`Delimiter`] for the new group
    ///
    /// ## Return Value
    /// Returns a new generator for the appended group
    pub fn group<'b>(&'b mut self, delimiter: Delimiter) -> Generator<'b> {
        self.group_at(delimiter, Span::call_site())
    }

    /// Appends a group to the stream
    ///
    /// ## Parameters
    ///  * `delimiter` - The [`Delimiter`] for the new group
    ///  * `span` - The [`Span`] for the appended group
    ///
    /// ## Return Value
    /// Returns a new generator for the appended group
    pub fn group_at<'b>(&'b mut self, delimiter: Delimiter, span: Span) -> Generator<'b> {
        self.tokens.push(OwnedTokenTree::Group(OwnedGroup::new(
            span,
            delimiter,
            Vec::new(),
        )));
        Generator::new(match self.tokens.last_mut().unwrap() {
            OwnedTokenTree::Group(group) => &mut group.tokens,
            _ => unreachable!(),
        })
    }
}
