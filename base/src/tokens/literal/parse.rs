use crate::{
    
    tokens::{Literal, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Literal(literal)) => Ok(literal.into()),
            _ => Err(parser.error("expected a literal")),
        }
    }
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Literal>().map(|literal| literal.clone())
    }
}
