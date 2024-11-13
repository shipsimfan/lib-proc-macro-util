use crate::{ast::VisibilityScope, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedVisibilityScope [
    EN => { "expected \"crate\", \"super\", \"self\", or \"in\"" },
    FR => { "« crate », « super », « self » ou « in » était attendu" },
    ZH => { "预期的 'crate'、'super'、'self' 或 'in'" },
]);

impl<'a> Parse<'a> for VisibilityScope<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(krate) = parser.step_parse() {
            return Ok(VisibilityScope::Crate(krate));
        }

        if let Ok(_self) = parser.step_parse() {
            return Ok(VisibilityScope::_Self(_self));
        }

        if let Ok(_super) = parser.step_parse() {
            return Ok(VisibilityScope::Super(_super));
        }

        Ok(VisibilityScope::Path(
            parser
                .parse()
                .map_err(|_| parser.error(m!(ExpectedVisibilityScope)))?,
            parser.parse()?,
        ))
    }
}
