use crate::{
    
    tokens::{Group, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Group(group)) => Ok(group),
            _ => Err(parser.error("expected a group")),
        }
    }
}

impl<'a> Parse<'a> for Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Group>().map(|group| group.clone())
    }
}
