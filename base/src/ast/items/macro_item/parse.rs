use crate::{ast::MacroItem, Parse, Parser, Result};

impl<'a> Parse<'a> for MacroItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(MacroItem::MacroInvocationSemi(macro_invocation));
        }

        Err(parser.error("expected a macro item"))
    }
}
