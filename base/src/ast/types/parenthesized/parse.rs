use crate::{ast::types::ParenthesizedType, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for ParenthesizedType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error("expected a parenthesized type"));
        }

        Ok(ParenthesizedType {
            r#type: parser.parse()?,
        })
    }
}
