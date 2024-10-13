use crate::{ast::types::ReferenceType, Parse, Parser, Result};

impl<'a> Parse<'a> for ReferenceType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ReferenceType {
            reference: parser.parse()?,
            lifetime: parser.parse()?,
            r#mut: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
