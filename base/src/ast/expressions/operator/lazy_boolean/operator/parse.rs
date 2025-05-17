use crate::{ast::expressions::LazyBooleanOperator, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for LazyBooleanOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(or) = parser.step_parse() {
            return Ok(LazyBooleanOperator::Or(or));
        }

        if let Ok(and) = parser.step_parse() {
            return Ok(LazyBooleanOperator::And(and));
        }

        Err(Error::new_at("unexpected token", parser.span()))
    }
}
