use crate::{
    ast::items::StructBody, supported_languages::*, tokens::Group, Delimiter, Error, Parse, Parser,
    Result,
};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedStructEnd [
    EN => { "expected the end of structure fields" },
    FR => { "la fin des champs de structure était attendue" },
    ZH => { "预期的结构字段结束" },
]);

i18n_translation::message_key!(ExpectedStructBody [
    EN => { "expected the structure body" },
    FR => { "le corps de la structure était attendu" },
    ZH => { "预期的结构体主体" },
]);

impl<'a> Parse<'a> for StructBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(group) = parser.step_parse::<&'a Group>() {
            let mut group_parser = group.parser();

            match group.delimiter {
                Delimiter::Parenthesis => {
                    let fields = group_parser.parse()?;
                    if !group_parser.empty() {
                        return Err(group_parser.error(m!(ExpectedStructEnd)));
                    }

                    return Ok(StructBody::Tuple {
                        fields,
                        where_clause: parser.parse()?,
                        semi: parser.parse()?,
                    });
                }
                Delimiter::Brace => {
                    let fields = group_parser.parse()?;
                    if !group_parser.empty() {
                        return Err(group_parser.error(m!(ExpectedStructEnd)));
                    }

                    return Ok(StructBody::Normal {
                        where_clause: None,
                        fields,
                    });
                }
                _ => return Err(Error::new_at(m!(ExpectedStructBody), group.span)),
            }
        }

        let where_clause = parser.parse()?;

        if let Ok(semi) = parser.step_parse() {
            return Ok(StructBody::Empty { where_clause, semi });
        }

        let group = parser.parse::<&'a Group>()?;
        if group.delimiter != Delimiter::Brace {
            return Err(Error::new_at(m!(ExpectedStructBody), group.span));
        }

        let mut group_parser = group.parser();
        let fields = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(group_parser.error(m!(ExpectedStructEnd)));
        }

        Ok(StructBody::Normal {
            where_clause,
            fields,
        })
    }
}
