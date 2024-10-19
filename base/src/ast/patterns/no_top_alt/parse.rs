use crate::{ast::PatternNoTopAlt, Parse, Parser, Result};

impl<'a> Parse<'a> for PatternNoTopAlt {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
