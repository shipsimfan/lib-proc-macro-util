use crate::{ast::items::EnumItems, Parse, Parser, Result};

impl<'a> Parse<'a> for EnumItems<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(EnumItems {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
