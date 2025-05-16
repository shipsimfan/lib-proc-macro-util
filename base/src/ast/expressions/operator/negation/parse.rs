use crate::{ast::expressions::NegationExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for NegationExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(NegationExpression {
            operator: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
