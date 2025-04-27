use crate::{ast::statements::LetStatement, Parse, Parser, Result};

impl<'a> Parse<'a> for LetStatement<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(LetStatement {
            attributes: parser.parse()?,
            r#let: parser.parse()?,
            pattern: parser.parse()?,
            r#type: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
