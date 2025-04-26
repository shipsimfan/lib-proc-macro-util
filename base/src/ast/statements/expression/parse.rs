use crate::{ast::statements::ExpressionStatement, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionStatement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block_expression) = parser.step_parse() {
            return Ok(ExpressionStatement::WithBlock(
                block_expression,
                parser.parse()?,
            ));
        }

        Ok(ExpressionStatement::WithoutBlock(
            parser.parse()?,
            parser.parse()?,
        ))
    }
}
