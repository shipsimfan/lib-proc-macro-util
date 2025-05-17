use crate::{ast::expressions::StructExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for StructExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructExpression {
            path: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
