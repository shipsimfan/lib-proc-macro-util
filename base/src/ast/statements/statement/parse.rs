use crate::{ast::Statement, Parse, Parser, Result};

impl<'a> Parse<'a> for Statement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Some(item) = parser.step_parse()? {
            return Ok(Statement::Item(item));
        }

        if let Some(r#let) = parser.step_parse()? {
            return Ok(Statement::Let(r#let));
        }

        if let Some(expression) = parser.step_parse()? {
            return Ok(Statement::Expression(expression));
        }

        if let Some(macro_invocation) = parser.step_parse()? {
            return Ok(Statement::MacroInvocation(macro_invocation));
        }

        Err(parser.error("expected expression, item or let statement"))
    }
}
