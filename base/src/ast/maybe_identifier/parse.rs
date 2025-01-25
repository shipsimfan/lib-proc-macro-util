use crate::{ast::MaybeIdentifier, Parse, Parser, Result};

impl<'a> Parse<'a> for MaybeIdentifier<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(identifier) = parser.step_parse() {
            return Ok(MaybeIdentifier::Identifier(identifier));
        }

        if let Ok(underscore) = parser.step_parse() {
            return Ok(MaybeIdentifier::Underscore(underscore));
        }

        Err(parser.error("expected a function parameter name"))
    }
}
