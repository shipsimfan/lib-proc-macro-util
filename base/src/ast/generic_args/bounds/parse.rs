use crate::{ast::GenericArgsBounds, Parse, Parser, Result};

impl<'a> Parse<'a> for GenericArgsBounds<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(GenericArgsBounds {
            identifier: parser.parse()?,
            args: parser.parse()?,
            colon: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}
