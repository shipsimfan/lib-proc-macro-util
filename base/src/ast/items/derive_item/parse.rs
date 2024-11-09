use crate::{ast::items::DeriveItem, Parse, Parser, Result};

impl<'a> Parse<'a> for DeriveItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(DeriveItem {
            attributes: parser.parse()?,
            visibility: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
