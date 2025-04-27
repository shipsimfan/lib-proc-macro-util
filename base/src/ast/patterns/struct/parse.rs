use crate::{ast::patterns::StructPattern, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for StructPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(Error::new_at(
                "a struct pattern body must be surrounded by braces",
                group.span,
            ));
        }

        let mut parser = group.parser();
        let elements = parser.parse()?;
        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(StructPattern { path, elements })
    }
}
