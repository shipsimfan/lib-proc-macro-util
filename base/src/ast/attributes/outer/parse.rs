use crate::{ast::OuterAttribute, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for OuterAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let hash = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            group.span.start().error("expected `[`").emit();
            return Err(());
        }

        let mut group_parser = group.parser();
        let attr = group_parser.parse()?;
        if !group_parser.empty() {
            group_parser.error("expected `]`").emit();
            return Err(());
        }

        Ok(OuterAttribute { hash, attr })
    }
}
