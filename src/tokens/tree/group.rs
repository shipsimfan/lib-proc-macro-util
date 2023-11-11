use crate::{parsing::Parser, tokens::OwnedGroup};
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

impl<'a> From<&'a OwnedGroup> for Group<'a> {
    fn from(value: &'a OwnedGroup) -> Self {
        Group {
            span: value.span,
            delimiter: value.delimiter,
            tokens: Parser::from(&value.tokens),
        }
    }
}
