use crate::{ast::expressions::LoopLabel, Parse, Parser, Result};

impl<'a> Parse<'a> for LoopLabel<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LoopLabel {
            quote: parser.parse()?,
            name: parser.parse()?,
            colon: parser.parse()?,
        })
    }
}
