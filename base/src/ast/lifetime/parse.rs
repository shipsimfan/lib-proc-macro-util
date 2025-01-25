use crate::{ast::Lifetime, Parse, Parser, Result};

impl<'a> Parse<'a> for Lifetime<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let quote = parser.parse()?;

        if let Ok(underscore) = parser.step_parse() {
            return Ok(Lifetime::Underscore(quote, underscore));
        }

        Ok(Lifetime::Identifier(
            quote,
            parser
                .parse()
                .map_err(|_| parser.error("expected a lifetime"))?,
        ))
    }
}
