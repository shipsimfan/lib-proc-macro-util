use crate::{ast::types::TraitObjectType, Parse, Parser, Result};

impl<'a> Parse<'a> for TraitObjectType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TraitObjectType {
            r#dyn: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}
