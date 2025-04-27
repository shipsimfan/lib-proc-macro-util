use crate::{ast::patterns::ReferencePattern, Parse, Parser, Result};

impl<'a> Parse<'a> for ReferencePattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ReferencePattern {
            ampersand: parser.parse()?,
            r#mut: parser.parse()?,
            pattern: parser.parse()?,
        })
    }
}
