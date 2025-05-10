use crate::{ast::expressions::ReturnExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ReturnExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ReturnExpression {
            r#return: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
