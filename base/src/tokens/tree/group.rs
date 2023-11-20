use crate::{parsing::Parser, tokens::OwnedGroup, Parse, ToTokens};
use proc_macro::{Delimiter, Span};

/// A delimited group of tokens
#[derive(Clone)]
pub struct Group<'a> {
    /// The span which covers this group
    pub span: Span,

    /// The delimiter dividing the group
    pub delimiter: Delimiter,

    /// The tokens contained by this buffer
    pub tokens: Parser<'a>,
}

impl<'a> Group<'a> {
    /// Gets the [`Span`] of this group
    ///
    /// ## Return Value
    /// Returns this group's [`Span`]
    pub fn span(&self) -> Span {
        self.span
    }

    /// Gets the [`Delimiter`] surrounding this group
    ///
    /// ## Return Value
    /// Returns the [`Delimiter`] surrounding this group
    pub fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    /// Gets the tokens in this group
    ///
    /// ## Return Value
    /// Returns the [`Parser`] for the tokens in this group
    pub fn tokens(&self) -> Parser<'a> {
        self.tokens.clone()
    }
}

impl<'a> From<&'a OwnedGroup> for Group<'a> {
    fn from(value: &'a OwnedGroup) -> Self {
        Group {
            span: value.span,
            delimiter: value.delimiter,
            tokens: Parser::from(&value.tokens),
        }
    }
}

impl<'a> Into<OwnedGroup> for Group<'a> {
    fn into(self) -> OwnedGroup {
        OwnedGroup {
            span: self.span,
            delimiter: self.delimiter,
            tokens: self.tokens.into(),
        }
    }
}

impl<'a> Parse<'a> for Group<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        parser.group().ok_or(parser.error("expected a group"))
    }
}

impl<'a> ToTokens for Group<'a> {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        generator
            .group_at(self.delimiter, self.span)
            .generate(&self.tokens);
    }
}
