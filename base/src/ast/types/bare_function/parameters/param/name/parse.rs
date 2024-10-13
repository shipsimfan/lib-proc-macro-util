use crate::{ast::types::MaybeNamedParamName, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_FUNCTION_PARAMETER_NAME [
    EN => { "expected a function parameter name" },
    FR => { "le nom d'un paramètre de fonction était attendu" },
    ZH => { "预期的函数参数名称" },
]);

impl<'a> Parse<'a> for MaybeNamedParamName {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(identifier) = parser.step_parse() {
            return Ok(MaybeNamedParamName::Identifier(identifier));
        }

        if let Ok(underscore) = parser.step_parse() {
            return Ok(MaybeNamedParamName::Underscore(underscore));
        }

        Err(parser.error(m!(EXPECTED_FUNCTION_PARAMETER_NAME)))
    }
}
