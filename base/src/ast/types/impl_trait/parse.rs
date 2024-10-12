use crate::{ast::types::ImplTraitType, Parse, Parser, Result};

impl<'a> Parse<'a> for ImplTraitType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ImplTraitType {
            r#impl: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}
