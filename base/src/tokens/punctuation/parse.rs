use crate::{
    tokens::{Punctuation, TokenTree},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for &'a Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let span = match parser.next() {
            Some(TokenTree::Punctuation(punctuation)) => return Ok(punctuation),
            Some(token_tree) => token_tree.span(),
            None => parser.span(),
        };

        span.error("expected punctuation").emit();
        Err(())
    }
}

impl<'a> Parse<'a> for Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Punctuation>()
            .map(|punctuation| punctuation.clone())
    }
}
