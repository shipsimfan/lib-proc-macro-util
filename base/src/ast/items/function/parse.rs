use crate::{
    ast::items::Function, supported_languages::*, tokens::Group, Delimiter, Error, Parse, Parser,
    Result,
};
use i18n::translation::m;

i18n::translation::message_key!(EXPECTED_FUNCTION_PARAMETERS [
    EN => { "expected function parameters" },
    FR => { "les paramètres de fonction étaient attendus" },
    ZH => { "预期的函数参数" },
]);

impl<'a> Parse<'a> for Function<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let qualifiers = parser.parse()?;
        let r#fn = parser.parse()?;
        let name = parser.parse()?;
        let generic_params = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at(m!(EXPECTED_FUNCTION_PARAMETERS), group.span));
        }

        let mut group_parser = group.parser();
        let parameters = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(Error::new_at(
                m!(EXPECTED_FUNCTION_PARAMETERS),
                group_parser.span(),
            ));
        }

        Ok(Function {
            qualifiers,
            r#fn,
            name,
            generic_params,
            parameters,
            return_type: parser.parse()?,
            where_clause: parser.parse()?,
            body: parser.parse()?,
        })
    }
}
