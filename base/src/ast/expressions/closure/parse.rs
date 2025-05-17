use crate::{ast::expressions::ClosureExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for ClosureExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ClosureExpression {
            r#async: parser.parse()?,
            r#move: parser.parse()?,
            leading: parser.parse()?,
            parameters: parser.parse()?,
            trailing: parser.parse()?,
            body: parser.parse()?,
        })
    }
}
