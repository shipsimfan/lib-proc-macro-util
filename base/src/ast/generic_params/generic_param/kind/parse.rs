use crate::{
    ast::{GenericParamKind, Lifetime},
    Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for GenericParamKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![const]>() {
            return parser
                .parse()
                .map(|r#const| GenericParamKind::Const(r#const));
        }

        if parser.peek::<Lifetime>() {
            return parser
                .parse()
                .map(|lifetime| GenericParamKind::Lifetime(lifetime));
        }

        if let Ok(r#type) = parser.step_parse() {
            return Ok(GenericParamKind::Type(r#type));
        }

        Err(parser.error("expected a generic paramter"))
    }
}
