use crate::{
    ast::{GenericParamKind, Lifetime},
    supported_languages::*,
    Parse, Parser, Result, Token,
};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_GENERIC_PARAMETER [
    EN => { "expected a generic paramter" },
    FR => { "un paramètre générique était attendu" },
    ZH => { "预期的泛型参数" },
]);

impl<'a> Parse<'a> for GenericParamKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![const]>() {
            return parser
                .parse()
                .map(|r#const| GenericParamKind::Const(r#const));
        }

        if parser.peek::<Lifetime>() {
            return parser
                .parse()
                .map(|lifetime| GenericParamKind::Lifetime(lifetime));
        }

        if let Ok(r#type) = parser.step_parse() {
            return Ok(GenericParamKind::Type(r#type));
        }

        Err(parser.error(m!(EXPECTED_GENERIC_PARAMETER)))
    }
}
