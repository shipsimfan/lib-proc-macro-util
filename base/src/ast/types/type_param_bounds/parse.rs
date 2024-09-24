use crate::{ast::TypeParamBounds, Parse, Parser, Result};

impl<'a> Parse<'a> for TypeParamBounds<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypeParamBounds {
            first: parser.parse()?,
            remaining: parser.parse()?,
            end: parser.parse()?,
        })
    }
}
