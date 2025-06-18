use crate::{tokens::TokenTree, Parser, Span};

impl<'a> From<(&'a [TokenTree], Span)> for Parser<'a> {
    fn from(stream: (&'a [TokenTree], Span)) -> Self {
        Parser::new(stream.0, stream.1)
    }
}
