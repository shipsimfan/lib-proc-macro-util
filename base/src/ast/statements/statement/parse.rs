use crate::{ast::Statement, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!( EXPECTED_STATEMENT [
    EN => { "expected a statement" },
    FR => { "une instruction était attendue" },
    ZH => { "预期的语句" },
]);

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Err(parser.error(m!(EXPECTED_STATEMENT)))
    }
}
