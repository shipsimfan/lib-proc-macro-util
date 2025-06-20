use crate::{ast::expressions::LazyBooleanOperator, Parse, Parser, Result};

impl<'a> Parse<'a> for LazyBooleanOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(or) = parser.step_parse() {
            return Ok(LazyBooleanOperator::Or(or));
        }

        if let Ok(and) = parser.step_parse() {
            return Ok(LazyBooleanOperator::And(and));
        }

        Err(parser.error("unexpected token"))
    }
}
