use crate::{ast::ExpressionWithBlock, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionWithBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ExpressionWithBlock {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
