use crate::{ast::items::FunctionParam, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = parser.parse()?;

        if let Ok(dots) = parser.step_parse() {
            return Ok(FunctionParam::Variadic { attributes, dots });
        }

        Ok(FunctionParam::Pattern {
            attributes,
            pattern: parser.parse()?,
            colon: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
