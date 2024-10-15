use crate::{ast::items::CrateRef, Parse, Parser, Result};

impl<'a> Parse<'a> for CrateRef<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser
            .parse()
            .map(|identifier| CrateRef::Identifier(identifier))
    }
}
