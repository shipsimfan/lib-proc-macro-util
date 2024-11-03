use crate::{ast::TypeNoBounds, supported_languages::*, Parse, Parser, Result};
use i18n::translation::m;

i18n::translation::message_key!(EXPECTED_TYPE_NO_BOUNDS [
    EN => { "expected a type with no bounds" },
    FR => { "un type sans contraintes était attendu" },
    ZH => { "预期的无边界类型" },
]);

impl<'a> Parse<'a> for TypeNoBounds<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(inferred) = parser.step_parse() {
            return Ok(TypeNoBounds::Inferred(inferred));
        }

        if let Ok(never) = parser.step_parse() {
            return Ok(TypeNoBounds::Never(never));
        }

        if let Ok(raw_pointer) = parser.step_parse() {
            return Ok(TypeNoBounds::RawPointer(raw_pointer));
        }

        if let Ok(reference) = parser.step_parse() {
            return Ok(TypeNoBounds::Reference(reference));
        }

        if let Ok(array) = parser.step_parse() {
            return Ok(TypeNoBounds::Reference(array));
        }

        if let Ok(slice) = parser.step_parse() {
            return Ok(TypeNoBounds::Slice(slice));
        }

        if let Ok(qualified_path) = parser.step_parse() {
            return Ok(TypeNoBounds::QualifiedPath(qualified_path));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(TypeNoBounds::MacroInvocation(macro_invocation));
        }

        if let Ok(tuple) = parser.step_parse() {
            return Ok(TypeNoBounds::Tuple(tuple));
        }

        if let Ok(parenthesized) = parser.step_parse() {
            return Ok(TypeNoBounds::Parenthesized(parenthesized));
        }

        if let Ok(bare_function) = parser.step_parse() {
            return Ok(TypeNoBounds::BareFunction(bare_function));
        }

        if let Ok(impl_trait) = parser.step_parse() {
            return Ok(TypeNoBounds::ImplTraitOneBound(impl_trait));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(TypeNoBounds::Path(path));
        }

        if let Ok(trait_object) = parser.step_parse() {
            return Ok(TypeNoBounds::TraitObjectOneBound(trait_object));
        }

        Err(parser.error(m!(EXPECTED_TYPE_NO_BOUNDS)))
    }
}
