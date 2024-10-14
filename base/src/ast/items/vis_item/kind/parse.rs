use crate::{ast::VisItemKind, Parse, Parser, Result};

impl<'a> Parse<'a> for VisItemKind {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
