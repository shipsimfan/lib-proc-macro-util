use crate::{ast::GenericArgs, Parse, Parser, Result};

impl<'a> Parse<'a> for GenericArgs<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(GenericArgs {
            open: parser.parse()?,
            args: parser.parse()?,
            last_arg: parser.parse()?,
            last_comma: parser.parse()?,
            end: parser.parse()?,
        })
    }
}
