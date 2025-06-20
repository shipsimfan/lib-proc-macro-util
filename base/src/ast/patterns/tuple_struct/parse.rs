use crate::{ast::patterns::TupleStructPattern, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for TupleStructPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group.span.start().error("expected `(`"));
        }

        let mut parser = group.parser();
        let items = parser.parse()?;
        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(TupleStructPattern { path, items })
    }
}
