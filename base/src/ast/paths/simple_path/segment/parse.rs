use crate::{ast::SimplePathSegment, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for SimplePathSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![$]>() {
            let dollar = parser.parse()?;
            let krate = parser.parse()?;

            return Ok(SimplePathSegment::DollarCrate(dollar, krate));
        }

        parser
            .parse()
            .map(|identifier| SimplePathSegment::Identifier(identifier))
            .map_err(|_| parser.error("expected a simple path segment"))
    }
}
