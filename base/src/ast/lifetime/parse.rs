use crate::{ast::Lifetime, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedLifetime [
    EN => { "expected a lifetime" },
    FR => { "une durée de vie était attendue" },
    ZH => { "预期的生命周期" },
]);

impl<'a> Parse<'a> for Lifetime<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let quote = parser.parse()?;

        if let Ok(underscore) = parser.step_parse() {
            return Ok(Lifetime::Underscore(quote, underscore));
        }

        Ok(Lifetime::Identifier(
            quote,
            parser
                .parse()
                .map_err(|_| parser.error(m!(ExpectedLifetime)))?,
        ))
    }
}
