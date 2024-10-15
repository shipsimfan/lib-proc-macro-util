use crate::{ast::items::UseDeclaration, Parse, Parser, Result};

impl<'a> Parse<'a> for UseDeclaration<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(UseDeclaration {
            r#use: parser.parse()?,
            tree: parser.parse()?,
            semi: parser.parse()?,
        })
    }
}
