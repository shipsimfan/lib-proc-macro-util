use crate::{ast::Expression, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_EXPRESSION [
    EN => { "expected an expression" },
    FR => { "une expression était attendue" },
    ZH => { "预期的表达式" },
]);

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(Expression::Literal(literal));
        }

        Err(parser.error(m!(EXPECTED_EXPRESSION)))
    }
}
