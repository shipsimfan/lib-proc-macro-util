use crate::{ast::expressions::IfExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for IfExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(IfExpression {
            r#if: parser.parse()?,
            r#let: parser.parse()?,
            condition: parser.parse()?,
            block: parser.parse()?,
            r#else: parser.parse()?,
        })
    }
}
