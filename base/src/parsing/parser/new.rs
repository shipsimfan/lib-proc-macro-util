use crate::{tokens::TokenTree, Parser};

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] over `stream`
    pub fn new(stream: &'a [TokenTree]) -> Self {
        Parser { stream, index: 0 }
    }
}
