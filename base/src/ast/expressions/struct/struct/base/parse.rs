use crate::{ast::expressions::StructBase, Parse, Parser, Result};

impl<'a> Parse<'a> for StructBase<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructBase {
            dots: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
