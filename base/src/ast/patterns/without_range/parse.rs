use crate::{ast::PatternWithoutRange, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for PatternWithoutRange<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step_parse() {
            return Ok(PatternWithoutRange::Literal(literal));
        }

        Err(Error::new_at(
            "expected a pattern without range",
            parser.span(),
        ))
    }
}
