use crate::{ast::TypeParam, Parse, Parser, Result};

impl<'a> Parse<'a> for TypeParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypeParam {
            identifier: parser.parse()?,
            bounds: parser.parse()?,
            default: parser.parse()?,
        })
    }
}
