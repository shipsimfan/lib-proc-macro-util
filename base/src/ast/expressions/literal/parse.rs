use crate::{ast::expressions::LiteralExpression, Parse, Parser, Result};

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
            .map_err(|_| parser.error("expected an attribute input"))
    }
}
