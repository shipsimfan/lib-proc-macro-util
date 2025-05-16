use crate::{ast::expressions::AsyncBlockExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for AsyncBlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(AsyncBlockExpression {
            r#async: parser.parse()?,
            r#move: parser.parse()?,
            block: parser.parse()?,
        })
    }
}
