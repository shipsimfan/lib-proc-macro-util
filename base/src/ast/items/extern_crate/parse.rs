use crate::{ast::items::ExternCrate, Parse, Parser, Result};

impl<'a> Parse<'a> for ExternCrate<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ExternCrate {
            r#extern: parser.parse()?,
            krate: parser.parse()?,
            crate_ref: parser.parse()?,
            as_clause: parser.parse()?,
        })
    }
}
