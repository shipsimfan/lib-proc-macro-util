use crate::{
    ast::Expression,
    supported_languages::{EN, FR},
    Parse, Parser, Result,
};
use i18n::m;

i18n::message_key!(EXPRESSION_EXPECTED [
    EN => { "expected an expression" },
    FR => { "attendu une expression" },
]);

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(Expression::Literal(literal));
        }

        Err(parser.error(m!(EXPRESSION_EXPECTED)))
    }
}
