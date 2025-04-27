use crate::{ast::patterns::SlicePatternItems, Parse, Parser, Result};

impl<'a> Parse<'a> for SlicePatternItems<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(SlicePatternItems {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
