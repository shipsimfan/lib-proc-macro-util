use crate::{
    ast::types::SliceType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_SLICE [
    EN => { "expected a slice" },
    FR => { "une tranche était attendue" },
    ZH => { "预期的切片" },
]);

impl<'a> Parse<'a> for SliceType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(parser.error(m!(EXPECTED_SLICE)));
        }

        Ok(SliceType {
            r#type: group.parser().parse()?,
        })
    }
}
