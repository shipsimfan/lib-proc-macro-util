use crate::{
    ast::{Meta, StructDeclaration},
    tokens::Struct,
    Generator, Parse, Result, ToTokens,
};

/// A declaration defining something
#[derive(Debug, Clone)]
pub enum Item<'a> {
    /// A definition of a [`Struct`]
    Struct(StructDeclaration<'a>),
}

impl<'a> Parse<'a> for Item<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
        let mut meta = Vec::new();
        while parser.peek::<Meta>() {
            meta.push(parser.parse()?);
        }

        let visibility = parser.parse()?;

        if parser.peek::<Struct>() {
            StructDeclaration::parse(meta, visibility, parser)
                .map(|r#struct| Item::Struct(r#struct))
        } else {
            Err(parser.error("expected a declaration"))
        }
    }
}

impl<'a> ToTokens for Item<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            Item::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
