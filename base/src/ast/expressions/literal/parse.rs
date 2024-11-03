use i18n::translation::m;

use crate::{ast::expressions::LiteralExpression, supported_languages::*, Parse, Parser, Result};

i18n::translation::message_key!( EXPECTED_ATTR_INPUT [
    EN => { "expected an attribute input" },
    FR => { "une entrée d'attribut était attendue" },
    ZH => { "预期的属性输入" },
]);

impl<'a> Parse<'a> for LiteralExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(r#true) = parser.step_parse() {
            return Ok(LiteralExpression::True(r#true));
        }

        if let Ok(r#false) = parser.step_parse() {
            return Ok(LiteralExpression::False(r#false));
        }

        parser
            .parse()
            .map(|literal| LiteralExpression::Literal(literal))
            .map_err(|_| parser.error(m!(EXPECTED_ATTR_INPUT)))
    }
}
