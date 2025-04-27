use crate::{ast::patterns::PathPattern, Parse, Parser, Result};

impl<'a> Parse<'a> for PathPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(PathPattern {
            path: parser.parse()?,
        })
    }
}
