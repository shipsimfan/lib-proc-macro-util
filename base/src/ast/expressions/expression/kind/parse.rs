use crate::{ast::ExpressionKind, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_EXPRESSION [
    EN => { "expected an expression" },
    FR => { "une expression était attendue" },
    ZH => { "预期的表达式" },
]);

impl<'a> Parse<'a> for ExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step_parse() {
            return Ok(ExpressionKind::Literal(literal));
        }

        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionKind::Block(block));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(ExpressionKind::Path(path));
        }

        Err(parser.error(m!(EXPECTED_EXPRESSION)))
    }
}
