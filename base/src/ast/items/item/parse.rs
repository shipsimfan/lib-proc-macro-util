use crate::{ast::Item, Parse, Parser, Result};

impl<'a> Parse<'a> for Item<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Item {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
