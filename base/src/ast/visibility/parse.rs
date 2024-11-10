use crate::{
    ast::Visibility, supported_languages::*, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!( EXPECTED_GROUP_WITH_PARENTHESIS [
    EN => { "expected a group delimited by parentheses" },
    FR => { "un groupe délimité par des parenthèses était attendu" },
    ZH => { "预期的由括号分隔的组" },
]);

impl<'a> Parse<'a> for Visibility<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#pub = parser.parse()?;
        let scope = match parser.step::<&'a Group, _>(Parser::parse) {
            Ok(group) => {
                if group.delimiter != Delimiter::Parenthesis {
                    return Err(Error::new_at(
                        m!(EXPECTED_GROUP_WITH_PARENTHESIS),
                        group.span,
                    ));
                }

                group.parser().parse()?
            }
            Err(_) => None,
        };

        Ok(Visibility { r#pub, scope })
    }
}
