use crate::{ast::expressions::GroupedExpression, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for GroupedExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = match parser.step_parse() {
            Ok(group) => group,
            Err(_) => return Err(parser.error("expected `(`")),
        };
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group.span.error("expected `(`"));
        }

        let mut parser = group.parser();

        let expression = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("expected `)`"));
        }

        Ok(GroupedExpression { expression })
    }
}
