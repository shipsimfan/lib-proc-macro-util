use crate::{ast::expressions::IfExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for IfExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
