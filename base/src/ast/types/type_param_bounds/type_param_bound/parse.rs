use crate::{ast::TypeParamBound, Parse, Parser, Result};

impl<'a> Parse<'a> for TypeParamBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse().map(TypeParamBound::Lifetime)
    }
}
