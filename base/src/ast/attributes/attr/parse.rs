use crate::{ast::Attr, tokens::Group, Parse, Parser, Result, Token};
use std::borrow::Cow;

impl<'a> Parse<'a> for Attr<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let path = parser.parse()?;
        let input = if parser.peek::<Token![=]>() || parser.peek::<Cow<'a, Group>>() {
            Some(parser.parse()?)
        } else {
            None
        };

        Ok(Attr { path, input })
    }
}
