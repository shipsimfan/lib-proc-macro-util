use crate::{tokens::TokenTree, Parser};

impl<'a> From<&'a [TokenTree]> for Parser<'a> {
    fn from(stream: &'a [TokenTree]) -> Self {
        Parser::new(stream)
    }
}
