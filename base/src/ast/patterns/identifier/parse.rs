use crate::{ast::patterns::IdentifierPattern, Parse, Parser, Result};

impl<'a> Parse<'a> for IdentifierPattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(IdentifierPattern {
            r#ref: parser.parse()?,
            r#mut: parser.parse()?,
            identifier: parser.parse()?,
            pattern: parser.parse()?,
        })
    }
}
