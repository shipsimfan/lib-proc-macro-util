use crate::{ast::TypePath, Parse, Parser, Result};

impl<'a> Parse<'a> for TypePath<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypePath {
            leading: parser.parse()?,
            first: parser.parse()?,
            remaining: parser.parse()?,
        })
    }
}
