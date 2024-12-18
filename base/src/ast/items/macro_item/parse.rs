use crate::{ast::MacroItem, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedMacroItem [
    EN => { "expected a macro item" },
    FR => { "un élément de macro était attendu" },
    ZH => { "预期的宏项目" },
]);

impl<'a> Parse<'a> for MacroItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(MacroItem::MacroInvocationSemi(macro_invocation));
        }

        Err(parser.error(m!(ExpectedMacroItem)))
    }
}
