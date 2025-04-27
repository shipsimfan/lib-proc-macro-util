use crate::{ast::patterns::WildcardPattern, Parse, Parser, Result};

impl<'a> Parse<'a> for WildcardPattern {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(WildcardPattern {
            underscore: parser.parse()?,
        })
    }
}
