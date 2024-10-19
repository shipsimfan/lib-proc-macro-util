use crate::{ast::items::SelfParam, Parse, Parser, Result};

impl<'a> Parse<'a> for SelfParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(SelfParam {
            attributes: parser.parse()?,
            reference: parser.parse()?,
            r#mut: parser.parse()?,
            _self: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
