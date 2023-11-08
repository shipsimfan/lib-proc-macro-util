use crate::{
    parsing::Parse,
    tokens::{Ident, Span},
};

#[derive(Clone)]
pub struct Lifetime {
    pub apostrophe: Span,
    pub ident: Ident,
}

impl<'a> Parse<'a> for Lifetime {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        parser.step(|parser| {
            parser
                .lifetime()
                .ok_or_else(|| parser.error("expected lifetime"))
        })
    }
}
