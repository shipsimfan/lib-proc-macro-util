use crate::{ast::patterns::RestPattern, Parse, Parser, Result};

impl<'a> Parse<'a> for RestPattern {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(RestPattern {
            dots: parser.parse()?,
        })
    }
}
