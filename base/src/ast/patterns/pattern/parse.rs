use crate::{ast::Pattern, Parse, Parser, Result};

impl<'a> Parse<'a> for Pattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Pattern {
            leading: parser.parse()?,
            first: parser.parse()?,
            remaining: parser.parse()?,
        })
    }
}
