use crate::{ast::expressions::LoopExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for LoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LoopExpression {
            label: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
