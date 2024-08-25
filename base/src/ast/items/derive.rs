use crate::{
    ast::{Meta, StructItem},
    tokens::Struct,
    Generator, Parse, Result, ToTokens,
};

/// A declaration which can have a trait derived for it
#[derive(Debug, Clone)]
pub enum Derive<'a> {
    /// A [`Struct`]
    Struct(StructItem<'a>),
}

impl<'a> Parse<'a> for Derive<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
        let mut meta = Vec::new();
        while parser.peek::<Meta>() {
            meta.push(parser.parse()?);
        }

        let visibility = parser.parse()?;

        if parser.peek::<Struct>() {
            StructItem::parse(meta, visibility, parser).map(|r#struct| Derive::Struct(r#struct))
        } else {
            Err(parser.error("expected `struct`"))
        }
    }
}

impl<'a> ToTokens for Derive<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            Derive::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
