use crate::{ast::patterns::StructPatternFields, Parse, Parser, Result};

impl<'a> Parse<'a> for StructPatternFields<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructPatternFields {
            first: parser.parse()?,
            remaining: parser.parse()?,
        })
    }
}
