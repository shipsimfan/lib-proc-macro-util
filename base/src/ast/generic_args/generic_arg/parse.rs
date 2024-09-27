use crate::{ast::GenericArg, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_GENERIC_ARG [
    EN => { "expected a generic argument" },
    FR => { "un argument générique était attendu" },
    ZH => { "预期的泛型参数" },
]);

impl<'a> Parse<'a> for GenericArg<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(lifetime) = parser.step(Parser::parse) {
            return Ok(GenericArg::Lifetime(lifetime));
        }

        if let Ok(r#type) = parser.step(Parser::parse) {
            return Ok(GenericArg::Binding(r#type));
        }

        if let Ok(r#type) = parser.step(Parser::parse) {
            return Ok(GenericArg::Bounds(r#type));
        }

        if let Ok(r#type) = parser.step(Parser::parse) {
            return Ok(GenericArg::Type(r#type));
        }

        if let Ok(r#const) = parser.step(Parser::parse) {
            return Ok(GenericArg::Const(r#const));
        }

        Err(parser.error(m!(EXPECTED_GENERIC_ARG)))
    }
}
