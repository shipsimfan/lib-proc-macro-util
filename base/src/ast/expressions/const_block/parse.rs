use crate::{ast::expressions::ConstBlockExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ConstBlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ConstBlockExpression {
            r#const: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
