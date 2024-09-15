use crate::{ast::Expression, Parse, Parser, Result};

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
