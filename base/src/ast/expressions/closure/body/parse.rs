use crate::{ast::expressions::ClosureBody, Parse, Parser, Result};

impl<'a> Parse<'a> for ClosureBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(arrow) = parser.step_parse() {
            return Ok(ClosureBody::ReturnType {
                arrow,
                r#type: parser.parse()?,
                expression: parser.parse()?,
            });
        }

        parser.parse().map(ClosureBody::Expression)
    }
}
