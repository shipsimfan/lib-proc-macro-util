use crate::{
    ast::patterns::GroupedPattern, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for GroupedPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at(
                "a grouped pattern must be surrounded by braces",
                group.span,
            ));
        }

        let mut parser = group.parser();
        let pattern = parser.parse()?;
        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(GroupedPattern { pattern })
    }
}
