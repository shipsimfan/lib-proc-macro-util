use crate::{
    ast::expressions::BlockExpression, supported_languages::*, tokens::Group, Delimiter, Parse,
    Parser, Result,
};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedBlockExpression [
    EN => { "expected a block expression" },
    FR => { "une expression de bloc était attendue" },
    ZH => { "预期的代码块表达式" },
]);

i18n_translation::message_key!( ExpectedClosingBrace [
    EN => { "expected \"}\" for block" },
    FR => { "« } » pour le bloc était attendu" },
    ZH => { "预期的用于块的 \"}\"" },
]);

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append(m!(ExpectedBlockExpression)))?;
        if group.delimiter != Delimiter::Brace {
            return Err(parser.error(m!(ExpectedBlockExpression)));
        }

        let mut group = group.parser();

        let attributes = group.parse()?;
        let statements = group.parse()?;
        let end = group.parse()?;

        if !parser.empty() {
            return Err(parser.error(m!(ExpectedClosingBrace)));
        }

        Ok(BlockExpression {
            attributes,
            statements,
            end,
        })
    }
}
