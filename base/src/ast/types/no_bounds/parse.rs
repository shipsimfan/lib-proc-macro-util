use crate::{ast::TypeNoBounds, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_TYPE_NO_BOUNDS [
    EN => { "expected a type with no bounds" },
    FR => { "un type sans contraintes était attendu" },
    ZH => { "预期的无边界类型" },
]);

impl<'a> Parse<'a> for TypeNoBounds<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(tuple) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::Tuple(tuple));
        }

        if let Ok(parenthesized) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::Parenthesized(parenthesized));
        }

        if let Ok(impl_trait) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::ImplTraitOneBound(impl_trait));
        }

        if let Ok(path) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::Path(path));
        }

        if let Ok(trait_object) = parser.step(Parser::parse) {
            return Ok(TypeNoBounds::TraitObjectOneBound(trait_object));
        }

        Err(parser.error(m!(EXPECTED_TYPE_NO_BOUNDS)))
    }
}
