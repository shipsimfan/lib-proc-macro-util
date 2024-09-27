use crate::{ast::GenericArgsBinding, Parse, Parser, Result};

impl<'a> Parse<'a> for GenericArgsBinding<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(GenericArgsBinding {
            identifier: parser.parse()?,
            args: parser.parse()?,
            equals: parser.parse()?,
            value: parser.parse()?,
        })
    }
}
