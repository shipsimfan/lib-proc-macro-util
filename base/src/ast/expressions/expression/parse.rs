use crate::{
    ast::Expression,
    supported_languages::{EN, FR},
    Parse, Parser, Result,
};
use i18n::translation::{Message, MessageKey};

const EXPRESSION_EXPECTED: MessageKey<()> = MessageKey::new(&[
    (EN, {
        fn english(_: &(), f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("expected an expression")
        }
        Message::new(english)
    }),
    (FR, {
        fn french(_: &(), f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("attendu une expression")
        }
        Message::new(french)
    }),
]);

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step(Parser::parse) {
            return Ok(Expression::Literal(literal));
        }

        Err(parser.error(EXPRESSION_EXPECTED.get(EN.clone()).display(&())))
    }
}
