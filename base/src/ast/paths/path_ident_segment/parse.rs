use crate::{ast::PathIdentSegment, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_PATH_SEGMENT [
    EN => { "expected a path segment" },
    FR => { "un segment de chemin était attendu" },
    ZH => { "预期的路径段" },
]);

impl<'a> Parse<'a> for PathIdentSegment {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dollar) = parser.step_parse() {
            return Ok(PathIdentSegment::DollarCrate(dollar, parser.parse()?));
        }

        parser
            .parse()
            .map(|identifier| PathIdentSegment::Identifier(identifier))
            .map_err(|_| parser.error(m!(EXPECTED_PATH_SEGMENT)))
    }
}
