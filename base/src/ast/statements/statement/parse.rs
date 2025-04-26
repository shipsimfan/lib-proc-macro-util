use crate::{ast::Statement, Parse, Parser, Result};

impl<'a> Parse<'a> for Statement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Some(item) = parser.step_parse()? {
            return Ok(Statement::Item(item));
        }

        if let Some(expression) = parser.step_parse()? {
            return Ok(Statement::Expression(expression));
        }

        Err(parser.error("expected a statement"))
    }
}
