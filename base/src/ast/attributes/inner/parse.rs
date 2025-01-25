use crate::{ast::InnerAttribute, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for InnerAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let hash = parser.parse()?;
        let exclamation = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(Error::new_at("expected an attribute", group.span));
        }

        let mut group_parser = group.parser();
        let attr = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(Error::new_at("expected an attribute", group_parser.span()));
        }

        Ok(InnerAttribute {
            hash,
            exclamation,
            attr,
        })
    }
}
