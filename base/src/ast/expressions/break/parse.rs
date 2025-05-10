use crate::{ast::expressions::BreakExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for BreakExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(BreakExpression {
            r#break: parser.parse()?,
            lifetime: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
