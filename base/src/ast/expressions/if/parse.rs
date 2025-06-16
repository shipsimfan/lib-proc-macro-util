use crate::{
    ast::{expressions::IfExpression, Expression},
    Parse, Parser, Result,
};

impl<'a> Parse<'a> for IfExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(IfExpression {
            r#if: parser.parse()?,
            r#let: parser.parse()?,
            condition: Box::new(Expression::parse_without_struct(parser)?),
            block: parser.parse()?,
            r#else: parser.parse()?,
        })
    }
}
