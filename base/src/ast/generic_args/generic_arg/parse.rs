use crate::{ast::GenericArg, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedGenericArg [
    EN => { "expected a generic argument" },
    FR => { "un argument générique était attendu" },
    ZH => { "预期的泛型参数" },
]);

impl<'a> Parse<'a> for GenericArg<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(lifetime) = parser.step_parse() {
            return Ok(GenericArg::Lifetime(lifetime));
        }

        if let Ok(r#type) = parser.step_parse() {
            return Ok(GenericArg::Binding(r#type));
        }

        if let Ok(r#type) = parser.step_parse() {
            return Ok(GenericArg::Bounds(r#type));
        }

        if let Ok(r#type) = parser.step_parse() {
            return Ok(GenericArg::Type(r#type));
        }

        if let Ok(r#const) = parser.step_parse() {
            return Ok(GenericArg::Const(r#const));
        }

        Err(parser.error(m!(ExpectedGenericArg)))
    }
}
