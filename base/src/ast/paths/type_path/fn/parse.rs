use crate::{
    ast::TypePathFn, supported_languages::*, tokens::Group, Delimiter, Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedFnInputs [
    EN => { "expected function inputs" },
    FR => { "les paramètres de la fonction étaient attendus" },
    ZH => { "预期的函数输入" },
]);
i18n_translation::message_key!(ExpectedEndOfFnInputs [
    EN => { "expected the end of function inputs" },
    FR => { "la fin des paramètres de la fonction était attendue" },
    ZH => { "预期的函数输入结束" },
]);

impl<'a> Parse<'a> for TypePathFn<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error(m!(ExpectedFnInputs)));
        }

        let mut group_parser = group.parser();
        let inputs = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(parser.error(m!(ExpectedEndOfFnInputs)));
        }

        let r#return = parser.parse()?;

        Ok(TypePathFn { inputs, r#return })
    }
}
