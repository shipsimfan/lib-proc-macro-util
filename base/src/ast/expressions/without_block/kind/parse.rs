use crate::{ast::ExpressionWithoutBlockKind, Parse, Parser, Result};

impl<'a> Parse<'a> for ExpressionWithoutBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(literal) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::Literal(literal));
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::MacroInvocation(
                macro_invocation,
            ));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::Path(path));
        }

        Err(parser.error("expected an expression without a block"))
    }
}
