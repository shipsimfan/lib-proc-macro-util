use crate::{ast::expressions::PredicateLoopExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for PredicateLoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(PredicateLoopExpression {
            r#while: parser.parse()?,
            r#let: parser.parse()?,
            condition: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
