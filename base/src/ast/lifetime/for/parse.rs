use crate::{ast::ForLifetimes, Parse, Parser, Result};

impl<'a> Parse<'a> for ForLifetimes<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ForLifetimes {
            r#for: parser.parse()?,
            generics: parser.parse()?,
        })
    }
}
