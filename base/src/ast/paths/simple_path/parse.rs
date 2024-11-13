use crate::{ast::SimplePath, supported_languages::*, Parse, Parser, Result, Token};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedSimplePath [
    EN => { "expected a simple path" },
    FR => { "un chemin simple était attendu" },
    ZH => { "预期的简单路径" },
]);

impl<'a> Parse<'a> for SimplePath<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let leading: Option<Token![::]> = parser.parse()?;
        let first = parser.parse().map_err(|error| {
            if leading.is_some() {
                error
            } else {
                parser.error(m!(ExpectedSimplePath))
            }
        })?;
        let remaining = parser.parse()?;

        Ok(SimplePath {
            leading,
            first,
            remaining,
        })
    }
}
