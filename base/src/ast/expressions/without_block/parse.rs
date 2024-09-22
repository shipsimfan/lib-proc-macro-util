use crate::{ast::ExpressionWithoutBlock, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ExpressionWithoutBlock {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
