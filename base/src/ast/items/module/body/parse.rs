use crate::{
    ast::items::ModuleBody, supported_languages::*, tokens::Group, Delimiter, Error, Parse, Parser,
    Result,
};
use i18n::m;

i18n::message_key!(EXPECTED_MODULE_BODY [
    EN => { "expected a module body" },
    FR => { "le corps du module était attendu" },
    ZH => { "预期的模块体" },
]);

impl<'a> Parse<'a> for ModuleBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(semi) = parser.step_parse() {
            return Ok(ModuleBody::None(semi));
        }

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(Error::new_at(m!(EXPECTED_MODULE_BODY), group.span));
        }

        let mut parser = group.parser();

        Ok(ModuleBody::Some {
            attributes: parser.parse()?,
            items: parser.parse()?,
        })
    }
}
