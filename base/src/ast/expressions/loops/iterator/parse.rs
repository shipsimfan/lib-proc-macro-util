use crate::{ast::expressions::IteratorLoopExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for IteratorLoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(IteratorLoopExpression {
            r#for: parser.parse()?,
            pattern: parser.parse()?,
            r#in: parser.parse()?,
            iterator: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
