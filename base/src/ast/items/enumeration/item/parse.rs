use crate::{ast::items::EnumItem, tokens::Group, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for EnumItem<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = parser.parse()?;
        let visibility = parser.parse()?;
        let name = parser.parse()?;

        let kind = if parser.peek::<&'a Group>() {
            Some(parser.parse()?)
        } else {
            None
        };

        let discriminant = if parser.peek::<Token![=]>() {
            Some(parser.parse()?)
        } else {
            None
        };

        Ok(EnumItem {
            attributes,
            visibility,
            name,
            kind,
            discriminant,
        })
    }
}
