use crate::{ast::patterns::RangePatternBound, Parse, Parser, Result};

impl<'a> Parse<'a> for RangePatternBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(path) = parser.step_parse() {
            return Ok(RangePatternBound::Path(path));
        }

        Ok(RangePatternBound::Literal(parser.parse()?, parser.parse()?))
    }
}
