use crate::{ast::VisItemKind, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for VisItemKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![use]>() {
            return parser.parse().map(|r#use| VisItemKind::Use(r#use));
        }

        if parser.peek::<Token![struct]>() {
            return parser.parse().map(|r#struct| VisItemKind::Struct(r#struct));
        }

        if parser.peek::<Token![enum]>() {
            return parser
                .parse()
                .map(|r#enum| VisItemKind::Enumeration(r#enum));
        }

        if parser.peek::<Token![mod]>() || parser.peek_n::<Token![mod]>(1) {
            return parser.parse().map(|module| VisItemKind::Module(module));
        }

        if parser.peek::<Token![extern]>() && parser.peek_n::<Token![crate]>(1) {
            return parser
                .parse()
                .map(|extern_crate| VisItemKind::ExternCrate(extern_crate));
        }

        if let Ok(function) = parser.step_parse() {
            return Ok(VisItemKind::Function(function));
        }

        Err(parser.error("expected an item"))
    }
}
