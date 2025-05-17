use crate::{ast::expressions::StructExprFieldName, Parse, Parser, Result};

impl<'a> Parse<'a> for StructExprFieldName<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step_parse() {
            return Ok(StructExprFieldName::Tuple(literal));
        }

        parser.parse().map(StructExprFieldName::Identifier)
    }
}
