use crate::{ast::TypePathFnInputs, Parse, Parser, Result};

impl<'a> Parse<'a> for TypePathFnInputs {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TypePathFnInputs {
            first: parser.parse()?,
            remaining: parser.parse()?,
            end: parser.parse()?,
        })
    }
}
