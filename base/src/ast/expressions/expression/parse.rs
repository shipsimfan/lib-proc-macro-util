use crate::{ast::Expression, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block_expression) = parser.step_parse() {
            return Ok(Expression::WithBlock(block_expression));
        }

        if let Ok(expression) = parser.step_parse() {
            return Ok(Expression::WithoutBlock(expression));
        }

        Err(Error::new_at("expected an expression", parser.span()))
    }
}
