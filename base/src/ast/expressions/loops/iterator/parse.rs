use crate::{
    ast::{expressions::IteratorLoopExpression, Expression},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for IteratorLoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(IteratorLoopExpression {
            r#for: parser.parse()?,
            pattern: parser.parse()?,
            r#in: parser.parse()?,
            iterator: Box::new(Expression::parse_without_struct(parser)?),
            block: parser.parse()?,
        })
    }
}
