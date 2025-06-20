use crate::{ast::expressions::TupleExpression, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for TupleExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = match parser.step_parse() {
            Ok(group) => group,
            Err(_) => return Err(parser.error("expected `(`")),
        };
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group.span.start().error("expected `(`"));
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
