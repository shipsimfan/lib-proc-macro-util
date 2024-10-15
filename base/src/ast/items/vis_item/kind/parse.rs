use crate::{ast::VisItemKind, supported_languages::*, Parse, Parser, Result, Token};
use i18n::m;

i18n::message_key!(EXPECTED_ITEM [
    EN => { "expected an item" },
    FR => { "un élément était attendu" },
    ZH => { "预期的条目" },
]);

impl<'a> Parse<'a> for VisItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![mod]>() || parser.peek_n::<Token![mod]>(1) {
            return parser.parse().map(|module| VisItemKind::Module(module));
        }

        Err(parser.error(m!(EXPECTED_ITEM)))
    }
}
