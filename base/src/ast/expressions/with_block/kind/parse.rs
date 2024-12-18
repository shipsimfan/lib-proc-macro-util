use crate::{ast::ExpressionWithBlockKind, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!( ExpectedExpessionWithBlock [
    EN => { "expected an expression with a block" },
    FR => { "une expression avec un bloc était attendue" },
    ZH => { "预期的带代码块的表达式" },
]);

impl<'a> Parse<'a> for ExpressionWithBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionWithBlockKind::Block(block));
        }

        Err(parser.error(m!(ExpectedExpessionWithBlock)))
    }
}
