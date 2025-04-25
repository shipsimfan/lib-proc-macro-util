use crate::{ast::expressions::DereferenceExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for DereferenceExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(DereferenceExpression {
            asterick: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
