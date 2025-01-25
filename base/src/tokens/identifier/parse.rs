use crate::{
    
    tokens::{Identifier, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Identifier(identifier)) => Ok(identifier.into()),
            _ => Err(parser.error("expected an identifier")),
        }
    }
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Identifier>()
            .map(|identifier| identifier.clone())
    }
}
