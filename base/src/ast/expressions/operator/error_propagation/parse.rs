use crate::{ast::expressions::ErrorPropagationExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ErrorPropagationExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ErrorPropagationExpression {
            expression: parser.parse()?,
            question: parser.parse()?,
        })
    }
}
