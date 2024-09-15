use crate::{ast::OuterAttribute, Parse, Parser, Result};

impl<'a> Parse<'a> for OuterAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(OuterAttribute {
            hash: parser.parse()?,
            attr: parser.parse()?,
        })
    }
}
