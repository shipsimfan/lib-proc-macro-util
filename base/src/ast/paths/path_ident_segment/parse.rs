use crate::{ast::PathIdentSegment, Parse, Parser, Result};

impl<'a> Parse<'a> for PathIdentSegment {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dollar) = parser.step_parse() {
            return Ok(PathIdentSegment::DollarCrate(dollar, parser.parse()?));
        }

        parser
            .parse()
            .map(|identifier| PathIdentSegment::Identifier(identifier))
            .map_err(|_| parser.error("expected a path segment"))
    }
}
