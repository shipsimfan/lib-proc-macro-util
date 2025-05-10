use crate::{ast::expressions::InfiniteLoopExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for InfiniteLoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(InfiniteLoopExpression {
            r#loop: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
