use crate::{ast::patterns::TuplePatternItems, Parse, Parser, Result};

impl<'a> Parse<'a> for TuplePatternItems<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(rest) = parser.step_parse() {
            return Ok(TuplePatternItems::Rest(rest));
        }

        let first = parser.parse()?;
        let first_comma = parser.parse()?;

        if let Ok(second) = parser.step_parse() {
            return Ok(TuplePatternItems::Multiple {
                first,
                first_comma,
                second,
                remaining: parser.parse()?,
                last: parser.parse()?,
            });
        }

        Ok(TuplePatternItems::Single(first, first_comma))
    }
}
