use crate::{ast::SimplePathSegment, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!( EXPECTED_SIMPLE_PATH_SEGMENT [
    EN => { "expected a simple path segment" },
    FR => { "un segment de chemin simple était attendu" },
    ZH => { "预期简单路径段" },
]);

impl<'a> Parse<'a> for SimplePathSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dollar) = parser.step_parse() {
            return Ok(SimplePathSegment::DollarCrate(dollar, parser.parse()?));
        }

        parser
            .parse()
            .map(|identifier| SimplePathSegment::Identifier(identifier))
            .map_err(|_| parser.error(m!(EXPECTED_SIMPLE_PATH_SEGMENT)))
    }
}
