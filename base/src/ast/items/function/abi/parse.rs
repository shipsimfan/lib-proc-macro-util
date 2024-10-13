use crate::{ast::Abi, Parse, Parser, Result};

impl<'a> Parse<'a> for Abi<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Abi(parser.parse()?))
    }
}
