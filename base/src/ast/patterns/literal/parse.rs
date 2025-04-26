use crate::{ast::patterns::LiteralPattern, Parse, Parser, Result};

impl<'a> Parse<'a> for LiteralPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(r#true) = parser.step_parse() {
            return Ok(LiteralPattern::True(r#true));
        }

        if let Ok(r#false) = parser.step_parse() {
            return Ok(LiteralPattern::False(r#false));
        }

        Ok(LiteralPattern::Literal(parser.parse()?, parser.parse()?))
    }
}
