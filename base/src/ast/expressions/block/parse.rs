use crate::{
    ast::expressions::BlockExpression, supported_languages::*, tokens::Group, Delimiter, Parse,
    Parser, Result,
};
use i18n::m;

i18n::message_key!( EXPECTED_BLOCK_EXPRESSION [
    EN => { "expected a block expression" },
    FR => { "une expression de bloc était attendue" },
    ZH => { "预期的代码块表达式" },
]);

i18n::message_key!( EXPECTED_CLOSING_BRACE [
    EN => { "expected \"}\" for block" },
    FR => { "« } » pour le bloc était attendu" },
    ZH => { "预期的用于块的 \"}\"" },
]);

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append(m!(EXPECTED_BLOCK_EXPRESSION)))?;
        if group.delimiter != Delimiter::Brace {
            return Err(parser.error(m!(EXPECTED_BLOCK_EXPRESSION)));
        }

        let mut group = group.parser();

        let attributes = group.parse()?;
        let statements = group.parse()?;
        let end = group.parse()?;

        if !parser.empty() {
            return Err(parser.error(m!(EXPECTED_CLOSING_BRACE)));
        }

        Ok(BlockExpression {
            attributes,
            statements,
            end,
        })
    }
}
