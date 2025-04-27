use crate::{ast::expressions::UnderscoreExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for UnderscoreExpression {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(UnderscoreExpression {
            underscore: parser.parse()?,
        })
    }
}
