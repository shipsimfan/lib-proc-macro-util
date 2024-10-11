use crate::{ast::PathInExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for PathInExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(PathInExpression {
            leading: parser.parse()?,
            first: parser.parse()?,
            remaining: parser.parse()?,
        })
    }
}
