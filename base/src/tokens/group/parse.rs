use crate::{
    tokens::{Group, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let span = match parser.next() {
            Some(TokenTree::Group(group)) => return Ok(group),
            Some(token_tree) => token_tree.span(),
            None => parser.span(),
        };

        span.error("expected a group").emit();
        Err(())
    }
}

impl<'a> Parse<'a> for Group {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<&'a Group>().map(|group| group.clone())
    }
}
