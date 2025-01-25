use crate::{ast::ExpressionKind, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step_parse() {
            return Ok(ExpressionKind::Literal(literal));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(ExpressionKind::MacroInvocation(macro_invocation));
        }

        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionKind::Block(block));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(ExpressionKind::Path(path));
        }

        Err(parser.error("expected an expression"))
    }
}
