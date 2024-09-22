use crate::{ast::MacroInvocation, Parse, Parser, Result};

impl<'a> Parse<'a> for MacroInvocation<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(MacroInvocation {
            path: parser.parse()?,
            exclamation: parser.parse()?,
            group: parser.parse()?,
        })
    }
}
