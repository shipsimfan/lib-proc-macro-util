use crate::{ast::Type, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_TYPE [
    EN => { "expected a type" },
    FR => { "un type était attendu" },
    ZH => { "预期的类型" },
]);

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(parenthesized) = parser.step(Parser::parse) {
            return Ok(Type::Parenthesized(parenthesized));
        }

        if let Ok(impl_trait_one_bound) = parser.step(Parser::parse) {
            return Ok(Type::ImplTraitOneBound(impl_trait_one_bound));
        }

        if let Ok(impl_trait) = parser.step(Parser::parse) {
            return Ok(Type::ImplTrait(impl_trait));
        }

        Err(parser.error(m!(EXPECTED_TYPE)))
    }
}
