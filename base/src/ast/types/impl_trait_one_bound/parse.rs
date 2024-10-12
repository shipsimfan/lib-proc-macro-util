use crate::{ast::types::ImplTraitTypeOneBound, Parse, Parser, Result};

impl<'a> Parse<'a> for ImplTraitTypeOneBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ImplTraitTypeOneBound {
            r#impl: parser.parse()?,
            bound: parser.parse()?,
        })
    }
}
