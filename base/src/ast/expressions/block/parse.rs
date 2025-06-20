use crate::{ast::expressions::BlockExpression, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = match parser.parse() {
            Ok(group) => group,
            Err(_) => return Err(parser.error("expected `{`")),
        };
        if group.delimiter != Delimiter::Brace {
            return Err(group.span.start().error("expected `{`"));
        }

        let mut parser = group.parser();

        let attributes = parser.parse()?;
        let statements = parser.parse()?;
        let end = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("expected `}`"));
        }

        Ok(BlockExpression {
            attributes,
            statements,
            end,
        })
    }
}
