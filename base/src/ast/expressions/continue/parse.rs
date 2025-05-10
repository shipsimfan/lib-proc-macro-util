use crate::{ast::expressions::ContinueExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ContinueExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ContinueExpression {
            r#continue: parser.parse()?,
            lifetime: parser.parse()?,
        })
    }
}
