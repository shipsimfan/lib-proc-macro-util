use crate::{ast::LifetimeBounds, Parse, Parser, Result};

impl<'a> Parse<'a> for LifetimeBounds<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LifetimeBounds {
            leading: parser.parse()?,
            ending: parser.parse()?,
        })
    }
}
