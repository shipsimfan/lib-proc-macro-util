use crate::{ast::expressions::LazyBooleanExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for LazyBooleanExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LazyBooleanExpression {
            left: parser.parse()?,
            operator: parser.parse()?,
            right: parser.parse()?,
        })
    }
}
