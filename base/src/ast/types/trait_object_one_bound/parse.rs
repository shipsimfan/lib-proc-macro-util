use crate::{ast::types::TraitObjectTypeOneBound, Parse, Parser, Result};

impl<'a> Parse<'a> for TraitObjectTypeOneBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TraitObjectTypeOneBound {
            r#dyn: parser.parse()?,
            bound: parser.parse()?,
        })
    }
}
