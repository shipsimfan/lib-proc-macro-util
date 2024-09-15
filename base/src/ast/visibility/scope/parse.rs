use crate::{ast::VisibilityScope, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for VisibilityScope<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(krate) = parser.step::<Token![crate], _>(Parser::parse) {
            return Ok(VisibilityScope::Crate(krate));
        }

        if let Ok(_self) = parser.step::<Token![self], _>(Parser::parse) {
            return Ok(VisibilityScope::_Self(_self));
        }

        if let Ok(_super) = parser.step::<Token![super], _>(Parser::parse) {
            return Ok(VisibilityScope::Super(_super));
        }

        Ok(VisibilityScope::Path(
            parser
                .parse()
                .map_err(|_| parser.error("expected \"crate\", \"super\", \"self\", or \"in\""))?,
            parser.parse()?,
        ))
    }
}
