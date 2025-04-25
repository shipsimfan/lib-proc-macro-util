use crate::{ast::expressions::OperatorExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for OperatorExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(borrow) = parser.step_parse() {
            return Ok(OperatorExpression::Borrow(borrow));
        }

        Err(parser.error("expected an opeator expression"))
    }
}
