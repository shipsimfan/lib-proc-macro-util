use crate::{ast::items::StructFields, Parse, Parser, Result};

impl<'a> Parse<'a> for StructFields<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructFields {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
