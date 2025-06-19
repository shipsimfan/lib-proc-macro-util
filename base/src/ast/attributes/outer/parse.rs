use crate::{ast::OuterAttribute, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for OuterAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let hash = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(group.span.start().error("expected `[`"));
        }

        let mut group_parser = group.parser();
        let attr = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(group_parser.error("expected `]`"));
        }

        Ok(OuterAttribute { hash, attr })
    }
}
