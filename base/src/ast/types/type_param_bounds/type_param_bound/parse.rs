use crate::{ast::TypeParamBound, Parse, Parser, Result};

impl<'a> Parse<'a> for TypeParamBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(lifetime) = parser.step_parse() {
            return Ok(TypeParamBound::Lifetime(lifetime));
        }

        parser.parse().map(TypeParamBound::Trait)
    }
}
