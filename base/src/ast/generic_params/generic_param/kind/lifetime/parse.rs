use crate::{ast::LifetimeParam, Parse, Parser, Result};

impl<'a> Parse<'a> for LifetimeParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LifetimeParam {
            lifetime: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}
