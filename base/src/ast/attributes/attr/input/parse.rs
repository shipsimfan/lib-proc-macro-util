use crate::{ast::AttrInput, Parse, Parser, Result};

impl<'a> Parse<'a> for AttrInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(equals) = parser.step_parse() {
            return Ok(AttrInput::Expression(equals, parser.parse()?));
        }

        if let Ok(group) = parser.step_parse() {
            return Ok(AttrInput::Group(group));
        }

        Err(parser.error("expected one of `(`, `::`, `=`, `[`, `]`, or `{`"))
    }
}
