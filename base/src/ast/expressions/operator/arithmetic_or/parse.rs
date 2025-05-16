use crate::{ast::expressions::ArithmeticOrLogicalExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ArithmeticOrLogicalExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ArithmeticOrLogicalExpression {
            left: parser.parse()?,
            operator: parser.parse()?,
            right: parser.parse()?,
        })
    }
}
