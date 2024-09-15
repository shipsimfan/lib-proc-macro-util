use crate::{ast::SimplePathSegment, Parse, Parser, Result};

impl<'a> Parse<'a> for SimplePathSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dollar) = parser.step(Parser::parse) {
            return Ok(SimplePathSegment::DollarCrate(dollar, parser.parse()?));
        }

        parser
            .parse()
            .map(|identifier| SimplePathSegment::Identifier(identifier))
            .map_err(|_| parser.error("expected a simple path segment"))
    }
}
