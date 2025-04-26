use crate::{ast::PatternNoTopAlt, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for PatternNoTopAlt<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(range) = parser.step_parse() {
            return Ok(PatternNoTopAlt::Range(range));
        }

        Err(Error::new_at("expected a pattern", parser.span()))
    }
}
