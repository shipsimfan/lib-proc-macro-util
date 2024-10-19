use crate::{ast::WhereClauseItem, Parse, Parser, Result};

impl<'a> Parse<'a> for WhereClauseItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(lifetime) = parser.step_parse() {
            return Ok(WhereClauseItem::Lifetime {
                lifetime,
                colon: parser.parse()?,
                bounds: parser.parse()?,
            });
        }

        Ok(WhereClauseItem::Type {
            for_lifetimes: parser.parse()?,
            r#type: parser.parse()?,
            colon: parser.parse()?,
            bounds: parser.parse()?,
        })
    }
}
