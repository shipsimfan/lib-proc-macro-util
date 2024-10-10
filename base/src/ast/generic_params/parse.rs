use crate::{ast::GenericParams, Parse, Parser, Result};

impl<'a> Parse<'a> for GenericParams<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(GenericParams {
            open: parser.parse()?,
            params: parser.parse()?,
            last_param: parser.parse()?,
            last_comma: parser.parse()?,
            close: parser.parse()?,
        })
    }
}
