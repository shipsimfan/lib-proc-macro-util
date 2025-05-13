use crate::{ast::items::DeriveItemKind, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for DeriveItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![struct]>() {
            return parser
                .parse()
                .map(|r#struct| DeriveItemKind::Struct(r#struct));
        }

        if parser.peek::<Token![enum]>() {
            return parser.parse().map(|r#enum| DeriveItemKind::Enum(r#enum));
        }

        Err(parser.error("expected an item"))
    }
}
