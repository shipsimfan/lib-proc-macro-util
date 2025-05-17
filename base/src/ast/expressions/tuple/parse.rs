use crate::{
    ast::expressions::TupleExpression, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for TupleExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append("expected a grouped expression"))?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at("expected a grouped expression", group.span));
        }

        let mut parser = group.parser();

        if parser.empty() {
            return Ok(TupleExpression {
                first: None,
                remaining: Vec::new(),
                last: None,
            });
        }

        let first = Some(parser.parse()?);
        let remaining = parser.parse()?;
        let last = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(TupleExpression {
            first,
            remaining,
            last,
        })
    }
}
