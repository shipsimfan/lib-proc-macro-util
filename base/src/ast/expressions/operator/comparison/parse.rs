use crate::{ast::expressions::ComparisonExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ComparisonExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ComparisonExpression {
            left: parser.parse()?,
            operator: parser.parse()?,
            right: parser.parse()?,
        })
    }
}
