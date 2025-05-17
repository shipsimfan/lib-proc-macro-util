use crate::{ast::expressions::StructExprStruct, Parse, Parser, Result};

impl<'a> Parse<'a> for StructExprStruct<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructExprStruct {
            first: parser.parse()?,
            remaining: parser.parse()?,
            base: parser.parse()?,
        })
    }
}
