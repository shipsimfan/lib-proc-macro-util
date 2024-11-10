use crate::{ast::types::RawPointerTypeMutability, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_MUT_OR_CONST [
    EN => { "expected `mut` or `const`" },
    FR => { "« mut » ou « const » était attendu" },
    ZH => { "预期的 `mut` 或 `const`" },
]);

impl<'a> Parse<'a> for RawPointerTypeMutability {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(r#mut) = parser.step_parse() {
            return Ok(RawPointerTypeMutability::Mut(r#mut));
        }

        if let Ok(r#const) = parser.step_parse() {
            return Ok(RawPointerTypeMutability::Const(r#const));
        }

        Err(parser.error(m!(EXPECTED_MUT_OR_CONST)))
    }
}
