use crate::{ast::patterns::GroupedPattern, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for GroupedPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group
                .span
                .start()
                .error("a grouped pattern must be surrounded by braces"));
        }

        let mut parser = group.parser();
        let pattern = parser.parse()?;
        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(GroupedPattern { pattern })
    }
}
