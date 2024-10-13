use crate::{
    ast::types::BareFunctionType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser,
    Result,
};
use i18n::m;

i18n::message_key!(EXPECTED_FUNCTION_PARAMETERS [
    EN => { "expected function parameters" },
    FR => { "les paramètres de fonction étaient attendus" },
    ZH => { "预期的函数参数" },
]);

impl<'a> Parse<'a> for BareFunctionType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let for_lifetimes = parser.parse()?;
        let qualifiers = parser.parse()?;
        let r#fn = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error(m!(EXPECTED_FUNCTION_PARAMETERS)));
        }

        let parameters = group.parser().parse()?;

        Ok(BareFunctionType {
            for_lifetimes,
            qualifiers,
            r#fn,
            parameters,
            return_type: parser.parse()?,
        })
    }
}
