use crate::{ast::expressions::CompoundAssignmentExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for CompoundAssignmentExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(CompoundAssignmentExpression {
            left: parser.parse()?,
            operator: parser.parse()?,
            right: parser.parse()?,
        })
    }
}
