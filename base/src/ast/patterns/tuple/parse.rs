use crate::{ast::patterns::TuplePattern, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for TuplePattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(Error::new_at(
                "a slice pattern must be surrounded by brackets",
                group.span,
            ));
        }

        let mut parser = group.parser();
        let items = parser.parse()?;
        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(TuplePattern { items })
    }
}
