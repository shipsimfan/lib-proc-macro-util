use crate::{
    tokens::{Literal, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let span = match parser.next() {
            Some(TokenTree::Literal(literal)) => return Ok(literal),
            Some(token_tree) => token_tree.span(),
            None => parser.span(),
        };

        Err(span.error("expected a literal"))
    }
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Literal>().map(|literal| literal.clone())
    }
}
