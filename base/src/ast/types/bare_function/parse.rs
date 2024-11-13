use crate::{
    ast::types::BareFunctionType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser,
    Result,
};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedFunctionParameters [
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
            return Err(parser.error(m!(ExpectedFunctionParameters)));
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
