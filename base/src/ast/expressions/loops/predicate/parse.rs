use crate::{
    ast::{expressions::PredicateLoopExpression, Expression},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for PredicateLoopExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(PredicateLoopExpression {
            r#while: parser.parse()?,
            r#let: parser.parse()?,
            condition: Box::new(Expression::parse_without_struct(parser)?),
            block: parser.parse()?,
        })
    }
}
