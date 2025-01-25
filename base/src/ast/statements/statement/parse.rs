use crate::{ast::Statement, Parse, Parser, Result};

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Err(parser.error("expected a statement"))
    }
}
