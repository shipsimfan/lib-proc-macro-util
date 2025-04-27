use crate::{ast::patterns::StructPatternField, Parse, Parser, Result};

impl<'a> Parse<'a> for StructPatternField<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructPatternField {
            attributes: parser.parse()?,
            name: parser.parse()?,
        })
    }
}
