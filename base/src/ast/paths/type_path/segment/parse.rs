use crate::{ast::TypePathSegment, Parse, Parser, Result};

impl<'a> Parse<'a> for TypePathSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypePathSegment {
            ident: parser.parse()?,
            generics: parser.parse()?,
        })
    }
}
