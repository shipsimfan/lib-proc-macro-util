use crate::{ast::PatternWithoutRange, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for PatternWithoutRange<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(wildcard) = parser.step_parse() {
            return Ok(PatternWithoutRange::Wildcard(wildcard));
        }

        if let Ok(literal) = parser.step_parse() {
            return Ok(PatternWithoutRange::Literal(literal));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(PatternWithoutRange::Identifier(identifier));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(PatternWithoutRange::MacroInvocation(macro_invocation));
        }

        Err(Error::new_at(
            "expected a pattern without range",
            parser.span(),
        ))
    }
}
