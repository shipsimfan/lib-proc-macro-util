use crate::{ast::VisItem, Parse, Parser, Result};

impl<'a> Parse<'a> for VisItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(VisItem {
            visibility: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
