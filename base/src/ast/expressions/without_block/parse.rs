use crate::{ast::ExpressionWithoutBlock, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionWithoutBlock<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(ExpressionWithoutBlock::Literal(literal));
        }

        Err(parser.error("expected an expression"))
    }
}
