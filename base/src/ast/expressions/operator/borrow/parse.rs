use crate::{ast::expressions::BorrowExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for BorrowExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(BorrowExpression {
            ampersand: parser.parse()?,
            mutable: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
