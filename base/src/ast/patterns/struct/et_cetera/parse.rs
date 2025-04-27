use crate::{ast::patterns::StructPatternEtCetera, Parse, Parser, Result};

impl<'a> Parse<'a> for StructPatternEtCetera<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructPatternEtCetera {
            attributes: parser.parse()?,
            dots: parser.parse()?,
        })
    }
}
