use crate::{ast::InnerAttribute, Parse, Parser, Result};

impl<'a> Parse<'a> for InnerAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(InnerAttribute {
            hash: parser.parse()?,
            exclamation: parser.parse()?,
            attr: parser.parse()?,
        })
    }
}
