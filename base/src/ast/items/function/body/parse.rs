use crate::{ast::items::FunctionBody, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(semi) = parser.step_parse() {
            return Ok(FunctionBody::Semi(semi));
        }

        Ok(FunctionBody::Expression(parser.parse()?))
    }
}
