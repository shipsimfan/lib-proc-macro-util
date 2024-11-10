use crate::{ast::items::DeriveItemKind, supported_languages::*, Parse, Parser, Result, Token};
use i18n_translation::m;

i18n_translation::message_key!(EXPECTED_ITEM [
    EN => { "expected an item" },
    FR => { "un élément était attendu" },
    ZH => { "预期的条目" },
]);

impl<'a> Parse<'a> for DeriveItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![struct]>() {
            return parser
                .parse()
                .map(|r#struct| DeriveItemKind::Struct(r#struct));
        }

        Err(parser.error(m!(EXPECTED_ITEM)))
    }
}
