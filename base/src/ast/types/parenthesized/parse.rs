use crate::{
    ast::types::ParenthesizedType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser,
    Result,
};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedParenthesizedType [
    EN => { "expected a parenthesized type" },
    FR => { "un type entre parenthèses était attendu" },
    ZH => { "预期的括号括起来的类型" },
]);

impl<'a> Parse<'a> for ParenthesizedType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error(m!(ExpectedParenthesizedType)));
        }

        Ok(ParenthesizedType {
            r#type: parser.parse()?,
        })
    }
}
