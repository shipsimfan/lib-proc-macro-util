use crate::{
    ast::types::ArrayType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_ARRAY [
    EN => { "expected an array" },
    FR => { "un tableau était attendu" },
    ZH => { "预期的数组" },
]);

impl<'a> Parse<'a> for ArrayType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(parser.error(m!(EXPECTED_ARRAY)));
        }

        Ok(ArrayType {
            r#type: parser.parse()?,
            semi: parser.parse()?,
            count: parser.parse()?,
        })
    }
}
