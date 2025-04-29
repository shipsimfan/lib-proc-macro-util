use crate::{tokens::TokenTree, Parse, Parser, Result};

impl<'a> Parse<'a> for &'a TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.next().ok_or(parser.error("expected a token tree"))
    }
}

impl<'a> Parse<'a> for TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a TokenTree>()
            .map(|token_tree| token_tree.clone())
    }
}
