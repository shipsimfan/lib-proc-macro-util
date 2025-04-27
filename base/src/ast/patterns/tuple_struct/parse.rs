use crate::{
    ast::patterns::TupleStructPattern, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for TupleStructPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at(
                "a tuple struct pattern body must be surrounded by parentheses",
                group.span,
            ));
        }

        let mut parser = group.parser();
        let items = parser.parse()?;
        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(TupleStructPattern { path, items })
    }
}
