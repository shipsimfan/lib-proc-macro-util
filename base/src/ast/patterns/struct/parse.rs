use crate::{ast::patterns::StructPattern, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for StructPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(group.span.start().error("expected `{`"));
        }

        let mut parser = group.parser();
        let elements = parser.parse()?;
        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(StructPattern { path, elements })
    }
}
