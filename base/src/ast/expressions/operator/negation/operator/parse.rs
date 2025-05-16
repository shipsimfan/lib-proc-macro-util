use crate::{ast::expressions::NegationOperator, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for NegationOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(dash) = parser.step_parse() {
            return Ok(NegationOperator::Numeric(dash));
        }

        if let Ok(exclamation) = parser.step_parse() {
            return Ok(NegationOperator::Boolean(exclamation));
        }

        Err(Error::new_at("unexpected token", parser.span()))
    }
}
