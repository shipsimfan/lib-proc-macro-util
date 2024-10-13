use crate::{ast::ConstParamValue, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!(EXPECTED_CONST_PARAM_VALUE [
    EN => { "expected a constant value" },
    FR => { "une valeur constante était attendue" },
    ZH => { "预期的常量值" },
]);

impl<'a> Parse<'a> for ConstParamValue<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block) = parser.step_parse() {
            return Ok(ConstParamValue::Block(block));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(ConstParamValue::Identifier(identifier));
        }

        Ok(ConstParamValue::Literal(
            parser.parse()?,
            parser
                .parse()
                .map_err(|error| error.append_at(m!(EXPECTED_CONST_PARAM_VALUE), parser.span()))?,
        ))
    }
}
