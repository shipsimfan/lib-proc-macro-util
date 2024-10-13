use crate::{ast::types::NeverType, Parse, Parser, Result};

impl<'a> Parse<'a> for NeverType {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(NeverType(parser.parse()?))
    }
}
