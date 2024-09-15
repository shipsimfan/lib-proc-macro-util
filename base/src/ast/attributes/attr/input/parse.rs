use crate::{ast::AttrInput, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for AttrInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![=]>() {
            return Ok(AttrInput::Expression(parser.parse()?, parser.parse()?));
        }

        parser
            .parse()
            .map(|group| AttrInput::Group(group))
            .map_err(|_| parser.error("expected a group or an '='"))
    }
}
