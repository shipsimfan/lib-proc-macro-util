use crate::{ast::SimplePathSegment, Parse, Parser, Result};

impl<'a> Parse<'a> for SimplePathSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dollar) = parser.step_parse() {
            return Ok(SimplePathSegment::DollarCrate(dollar, parser.parse()?));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(SimplePathSegment::Identifier(identifier));
        }

        Err(parser.error("expected identifier, `self`, `super`, `crate`, or `Self`"))
    }
}
