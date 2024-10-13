use crate::{
    ast::types::TupleType, supported_languages::*, tokens::Group, Delimiter, Parse, Parser,
};
use i18n::m;

i18n::message_key!(EXPECTED_TUPLE [
    EN => { "expected a tuple" },
    FR => { "un tuple était attendu" },
    ZH => { "预期的元组" },
]);

impl<'a> Parse<'a> for TupleType<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error(m!(EXPECTED_TUPLE)));
        }

        let mut parser = group.parser();

        Ok(TupleType {
            types: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
