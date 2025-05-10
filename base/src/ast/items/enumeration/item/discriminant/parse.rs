use crate::{ast::items::EnumItemDiscriminant, Parse, Parser, Result};

impl<'a> Parse<'a> for EnumItemDiscriminant<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(EnumItemDiscriminant {
            equals: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
