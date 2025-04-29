use crate::{ast::types::SliceType, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for SliceType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(parser.error("expected a slice"));
        }

        Ok(SliceType {
            r#type: group.parser().parse()?,
        })
    }
}
