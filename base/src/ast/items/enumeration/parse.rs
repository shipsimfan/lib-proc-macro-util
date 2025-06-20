use crate::{ast::items::Enumeration, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for Enumeration<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#enum = parser.parse()?;
        let name = parser.parse()?;
        let generic_params = parser.parse()?;
        let where_clause = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(group
                .span
                .start()
                .error("enum body must be surrounded by braces"));
        }
        let mut parser = group.parser();
        let enum_items = if parser.empty() {
            None
        } else {
            Some(parser.parse()?)
        };
        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(Enumeration {
            r#enum,
            name,
            generic_params,
            where_clause,
            enum_items,
        })
    }
}
