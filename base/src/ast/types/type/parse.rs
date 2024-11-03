use crate::{ast::Type, supported_languages::*, Parse, Parser, Result};
use i18n::translation::m;

i18n::translation::message_key!(EXPECTED_TYPE [
    EN => { "expected a type" },
    FR => { "un type était attendu" },
    ZH => { "预期的类型" },
]);

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(inferred) = parser.step_parse() {
            return Ok(Type::Inferred(inferred));
        }

        if let Ok(never) = parser.step_parse() {
            return Ok(Type::Never(never));
        }

        if let Ok(raw_pointer) = parser.step_parse() {
            return Ok(Type::RawPointer(raw_pointer));
        }

        if let Ok(reference) = parser.step_parse() {
            return Ok(Type::Reference(reference));
        }

        if let Ok(array) = parser.step_parse() {
            return Ok(Type::Array(array));
        }

        if let Ok(slice) = parser.step_parse() {
            return Ok(Type::Slice(slice));
        }

        if let Ok(qualified_path) = parser.step_parse() {
            return Ok(Type::QualifiedPath(qualified_path));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(Type::MacroInvocation(macro_invocation));
        }

        if let Ok(tuple) = parser.step_parse() {
            return Ok(Type::Tuple(tuple));
        }

        if let Ok(parenthesized) = parser.step_parse() {
            return Ok(Type::Parenthesized(parenthesized));
        }

        if let Ok(bare_function) = parser.step_parse() {
            return Ok(Type::BareFunction(bare_function));
        }

        if let Ok(impl_trait_one_bound) = parser.step_parse() {
            return Ok(Type::ImplTraitOneBound(impl_trait_one_bound));
        }

        if let Ok(impl_trait) = parser.step_parse() {
            return Ok(Type::ImplTrait(impl_trait));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(Type::Path(path));
        }

        if let Ok(trait_object_one_bound) = parser.step_parse() {
            return Ok(Type::TraitObjectOneBound(trait_object_one_bound));
        }

        if let Ok(trait_object) = parser.step_parse() {
            return Ok(Type::TraitObject(trait_object));
        }

        Err(parser.error(m!(EXPECTED_TYPE)))
    }
}
