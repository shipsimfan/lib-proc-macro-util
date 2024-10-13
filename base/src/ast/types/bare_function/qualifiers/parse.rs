use crate::{ast::types::FunctionTypeQualifiers, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionTypeQualifiers<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(FunctionTypeQualifiers {
            r#unsafe: parser.parse()?,
            r#extern: parser.parse()?,
        })
    }
}
