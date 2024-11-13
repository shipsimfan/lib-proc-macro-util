use crate::{ast::Attr, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedAttribute [
    EN => { "expected an attribute" },
    FR => { "un attribut était attendu" },
    ZH => { "预期的属性" },
]);

impl<'a> Parse<'a> for Attr<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Attr {
            path: parser
                .parse()
                .map_err(|error| error.append(m!(ExpectedAttribute)))?,
            input: parser.parse()?,
        })
    }
}
