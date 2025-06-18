use crate::{tokens::TokenTree, Parser, Span};

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] over `stream`
    pub fn new(stream: &'a [TokenTree], span: Span) -> Self {
        Parser {
            stream,
            index: 0,
            span,
        }
    }
}
