use crate::{ast::items::Struct, Parse, Parser, Result};

impl<'a> Parse<'a> for Struct<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Struct {
            r#struct: parser.parse()?,
            name: parser.parse()?,
            generic_params: parser.parse()?,
            body: parser.parse()?,
        })
    }
}
