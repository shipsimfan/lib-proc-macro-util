use crate::tokens::Punctuation;

impl<'a> Parse<'a> for &'a Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        match parser.next() {
            Some(TokenTree::Punctuation(punctuation)) => Ok(punctuation.into()),
            _ => Err(parser.error("expected punctuation")),
        }
    }
}

impl<'a> Parse<'a> for Punctuation {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse::<&'a Punctuation>()
            .map(|literal| literal.clone())
    }
}
