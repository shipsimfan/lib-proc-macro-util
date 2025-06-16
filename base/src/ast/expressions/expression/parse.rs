use crate::{
    ast::{Expression, ExpressionWithoutBlock},
    Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Self::do_parse(parser, true)
    }
}

impl<'a> Expression<'a> {
    /// Parse an [`Expression`] without parsing a
    /// [`StructExpression`](crate::ast::expressions::StructExpression)
    pub fn parse_without_struct(parser: &mut Parser<'a>) -> Result<Self> {
        Self::do_parse(parser, false)
    }

    fn do_parse(parser: &mut Parser<'a>, include_struct: bool) -> Result<Self> {
        if let Ok(block_expression) = parser.step_parse() {
            return Ok(Expression::WithBlock(block_expression));
        }

        if let Ok(expression) =
            parser.step(|parser| ExpressionWithoutBlock::do_parse(parser, include_struct))
        {
            return Ok(Expression::WithoutBlock(expression));
        }

        Err(Error::new_at("expected an expression", parser.span()))
    }
}
