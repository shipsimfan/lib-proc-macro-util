use crate::{ast::items::Module, Parse, Parser, Result};

impl<'a> Parse<'a> for Module<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Module {
            r#unsafe: parser.parse()?,
            r#mod: parser.parse()?,
            identifier: parser.parse()?,
            body: parser.parse()?,
        })
    }
}
