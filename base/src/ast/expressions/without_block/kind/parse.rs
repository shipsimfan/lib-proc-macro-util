use crate::{ast::ExpressionWithoutBlockKind, supported_languages::*, Parse, Parser, Result};
use i18n::m;

i18n::message_key!( EXPECTED_EXPESSION_WITHOUT_BLOCK [
    EN => { "expected an expression without a block" },
    FR => { "une expression sans bloc était attendue" },
    ZH => { "不含代码块的预期表达式" },
]);

impl<'a> Parse<'a> for ExpressionWithoutBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(ExpressionWithoutBlockKind::Literal(literal));
        }

        Err(parser.error(m!(EXPECTED_EXPESSION_WITHOUT_BLOCK)))
    }
}