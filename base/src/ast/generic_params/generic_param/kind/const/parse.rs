use crate::{ast::ConstParam, Parse, Parser, Result};

impl<'a> Parse<'a> for ConstParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ConstParam {
            r#const: parser.parse()?,
            identifier: parser.parse()?,
            colon: parser.parse()?,
            r#type: parser.parse()?,
            value: parser.parse()?,
        })
    }
}
