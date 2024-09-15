use crate::{ast::Expression, Parse, Parser, Result};

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(Expression::Literal(literal));
        }

        Err(parser.error("expected an expression"))
    }
}
