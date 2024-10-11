use crate::{ast::PathExprSegment, Parse, Parser, Result};

impl<'a> Parse<'a> for PathExprSegment<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(PathExprSegment {
            ident: parser.parse()?,
            generic_args: parser.parse()?,
        })
    }
}
