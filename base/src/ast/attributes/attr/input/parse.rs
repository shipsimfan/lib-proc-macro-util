use crate::{ast::AttrInput, supported_languages::*, Parse, Parser, Result};
use i18n::translation::m;

i18n::translation::message_key!( EXPECTED_ATTR_INPUT [
    EN => { "expected a group or an '='" },
    FR => { "un groupe ou un '=' était attendu" },
    ZH => { "预期的组或 '='" },
]);

impl<'a> Parse<'a> for AttrInput<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(equals) = parser.step_parse() {
            return Ok(AttrInput::Expression(equals, parser.parse()?));
        }

        parser
            .parse()
            .map(|group| AttrInput::Group(group))
            .map_err(|_| parser.error(m!(EXPECTED_ATTR_INPUT)))
    }
}
