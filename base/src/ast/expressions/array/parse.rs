use crate::{ast::expressions::ArrayExpression, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for ArrayExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = match parser.step_parse() {
            Ok(group) => group,
            Err(_) => return Err(parser.error("expected `[`")),
        };
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group.span.start().error("expected `[`"));
        }

        let mut parser = group.parser();

        let elements = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("expected `]`"));
        }

        Ok(ArrayExpression { elements })
    }
}
