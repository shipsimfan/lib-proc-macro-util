use crate::{ast::QualifiedPathType, Parse, Parser, Result};

impl<'a> Parse<'a> for QualifiedPathType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(QualifiedPathType {
            start: parser.parse()?,
            r#type: parser.parse()?,
            r#as: parser.parse()?,
            end: parser.parse()?,
        })
    }
}
