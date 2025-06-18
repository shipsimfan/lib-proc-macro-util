use crate::{tokens::TokenTree, Parse, Parser, Result};

impl<'a> Parse<'a> for &'a TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(token_tree) => Ok(token_tree),
            None => {
                parser.error("expected a token tree").emit();
                Err(())
            }
        }
    }
}

impl<'a> Parse<'a> for TokenTree {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a TokenTree>()
            .map(|token_tree| token_tree.clone())
    }
}
