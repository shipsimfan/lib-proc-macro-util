use crate::{
    ast::{Function, Meta, StructItem},
    Generator, Parse, Result, ToTokens, Token,
};

/// A declaration defining something
#[derive(Debug, Clone)]
pub enum Item<'a> {
    /// A definition of a [`Struct`]
    Struct(StructItem<'a>),

    /// A definition of a function
    Fn(Function<'a>),
}

impl<'a> Parse<'a> for Item<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
        let mut meta = Vec::new();
        while parser.peek::<Meta>() {
            meta.push(parser.parse()?);
        }

        let visibility = parser.parse()?;

        if parser.peek::<Token![struct]>() {
            StructItem::parse(meta, visibility, parser).map(|r#struct| Item::Struct(r#struct))
        } else if parser.peek::<Token![fn]>() {
            Function::parse(meta, visibility, parser).map(|r#fn| Item::Fn(r#fn))
        } else {
            Err(parser.error("expected an item"))
        }
    }
}

impl<'a> ToTokens for Item<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            Item::Struct(r#struct) => r#struct.to_tokens(generator),
            Item::Fn(r#fn) => r#fn.to_tokens(generator),
        }
    }
}
