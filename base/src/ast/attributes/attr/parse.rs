use crate::{ast::Attr, Parse, Parser, Result};

impl<'a> Parse<'a> for Attr<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Attr {
            path: parser
                .parse()
                .map_err(|error| error.append("expected an attribute path"))?,
            input: parser.parse()?,
        })
    }
}
