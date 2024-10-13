use crate::{ast::types::MaybeNamedParam, Parse, Parser, Result};

impl<'a> Parse<'a> for MaybeNamedParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(MaybeNamedParam {
            attributes: parser.parse()?,
            name: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
