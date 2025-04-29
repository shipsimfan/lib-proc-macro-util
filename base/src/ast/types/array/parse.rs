use crate::{ast::types::ArrayType, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for ArrayType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(parser.error("expected an array"));
        }

        Ok(ArrayType {
            r#type: parser.parse()?,
            semi: parser.parse()?,
            count: parser.parse()?,
        })
    }
}
