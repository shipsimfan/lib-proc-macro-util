use crate::{ast::patterns::TuplePattern, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for TuplePattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(group.span.start().error("expected `{`"));
        }

        let mut parser = group.parser();
        let items = parser.parse()?;
        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(TuplePattern { items })
    }
}
