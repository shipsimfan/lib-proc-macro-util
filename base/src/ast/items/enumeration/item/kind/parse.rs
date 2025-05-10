use crate::{ast::items::EnumItemKind, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for EnumItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        let mut parser = group.parser();
        let ret = match group.delimiter {
            Delimiter::Brace => EnumItemKind::Struct(if parser.empty() {
                None
            } else {
                Some(parser.parse()?)
            }),
            Delimiter::Parenthesis => EnumItemKind::Tuple(if parser.empty() {
                None
            } else {
                Some(parser.parse()?)
            }),
            _ => return Err(Error::new_at("unexpected token", group.span)),
        };
        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }
        Ok(ret)
    }
}
