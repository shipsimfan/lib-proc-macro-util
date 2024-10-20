use crate::{ast::items::FunctionQualifiers, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionQualifiers<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(FunctionQualifiers {
            r#const: parser.parse()?,
            r#async: parser.parse()?,
            r#unsafe: parser.parse()?,
            r#extern: parser.parse()?,
        })
    }
}
