use crate::{ast::TypeNoBounds, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_TYPE_NO_BOUNDS [
    EN => { "expected a type with no bounds" },
    FR => { "un type sans contraintes était attendu" },
    ZH => { "预期的无边界类型" },
]);

impl<'a> Parse<'a> for TypeNoBounds {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(parenthesized) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::Parenthesized(parenthesized));
        }

        Err(parser.error(m!(EXPECTED_TYPE_NO_BOUNDS)))
    }
}
