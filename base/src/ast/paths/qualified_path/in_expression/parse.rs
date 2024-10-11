use crate::{ast::QualifiedPathInExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for QualifiedPathInExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(QualifiedPathInExpression {
            r#type: parser.parse()?,
            segments: parser.parse()?,
        })
    }
}
