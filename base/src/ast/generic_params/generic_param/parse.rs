use crate::{ast::GenericParam, Parse, Parser, Result};

impl<'a> Parse<'a> for GenericParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(GenericParam {
            attributes: parser.parse()?,
            kind: parser.parse()?,
        })
    }
}
