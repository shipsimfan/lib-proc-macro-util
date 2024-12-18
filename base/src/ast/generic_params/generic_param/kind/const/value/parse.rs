use crate::{ast::ConstParamValue, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedConstParamValue [
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
                .map_err(|error| error.append_at(m!(ExpectedConstParamValue), parser.span()))?,
        ))
    }
}
