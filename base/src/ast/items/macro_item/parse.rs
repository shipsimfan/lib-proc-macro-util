use crate::{ast::MacroItem, Parse, Parser, Result};

impl<'a> Parse<'a> for MacroItem {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
