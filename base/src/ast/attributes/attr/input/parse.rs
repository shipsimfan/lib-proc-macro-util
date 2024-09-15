use crate::{ast::AttrInput, Parse, Parser, Result};

impl<'a> Parse<'a> for AttrInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(equals) = parser.step(Parser::parse) {
            return Ok(AttrInput::Expression(equals, parser.parse()?));
        }

        parser
            .parse()
            .map(|group| AttrInput::Group(group))
            .map_err(|_| parser.error("expected a group or an '='"))
    }
}
