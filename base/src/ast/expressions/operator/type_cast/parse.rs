use crate::{ast::expressions::TypeCastExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for TypeCastExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypeCastExpression {
            expression: parser.parse()?,
            r#as: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
