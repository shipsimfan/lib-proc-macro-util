use crate::{ast::TypePathSegmentGenerics, Parse, Parser, Result};

impl<'a> Parse<'a> for TypePathSegmentGenerics<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(type_path_fn) = parser.step(Parser::parse) {
            return Ok(TypePathSegmentGenerics::TypePathFn(type_path_fn));
        }

        todo!()
    }
}