use crate::{ast::expressions::StructExprTuple, Parse, Parser, Result};

impl<'a> Parse<'a> for StructExprTuple<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructExprTuple {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
