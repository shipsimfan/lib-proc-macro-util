use crate::{ast::WhereClause, Parse, Parser, Result};

impl<'a> Parse<'a> for WhereClause<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(WhereClause {
            r#where: parser.parse()?,
            initial: parser.parse()?,
            r#final: parser.parse()?,
        })
    }
}
