use crate::{
    ast::InnerAttribute, supported_languages::*, tokens::Group, Delimiter, Error, Parse, Parser,
    Result,
};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedAttribute [
    EN => { "expected an attribute" },
    FR => { "un attribut était attendu" },
    ZH => { "预期的属性" },
]);

impl<'a> Parse<'a> for InnerAttribute<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let hash = parser.parse()?;
        let exclamation = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Bracket {
            return Err(Error::new_at(m!(ExpectedAttribute), group.span));
        }

        let mut group_parser = group.parser();
        let attr = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(Error::new_at(m!(ExpectedAttribute), group_parser.span()));
        }

        Ok(InnerAttribute {
            hash,
            exclamation,
            attr,
        })
    }
}
