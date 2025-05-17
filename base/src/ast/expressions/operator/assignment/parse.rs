use crate::{ast::expressions::AssignmentExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for AssignmentExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(AssignmentExpression {
            left: parser.parse()?,
            equals: parser.parse()?,
            right: parser.parse()?,
        })
    }
}
