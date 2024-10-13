use crate::{ast::types::RawPointerType, Parse, Parser, Result};

impl<'a> Parse<'a> for RawPointerType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(RawPointerType {
            asterick: parser.parse()?,
            mutability: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
