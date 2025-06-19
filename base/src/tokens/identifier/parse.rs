use crate::{
    tokens::{Identifier, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let span = match parser.next() {
            Some(TokenTree::Identifier(identifier)) => return Ok(identifier),
            Some(token_tree) => token_tree.span(),
            None => parser.span(),
        };

        Err(span.error("expected an identifier"))
    }
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Identifier>()
            .map(|identifier| identifier.clone())
    }
}
