use crate::{ast::expressions::UnsafeBlockExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for UnsafeBlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(UnsafeBlockExpression {
            r#unsafe: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
