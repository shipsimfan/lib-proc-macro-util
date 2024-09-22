use crate::{ast::Expression, Parse, Parser, Result};

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Expression {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
