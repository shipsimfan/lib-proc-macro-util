use crate::{ast::StructDeclaration, tokens::Struct, Generator, Parse, Result, ToTokens};

/// A declaration which can have a trait derived for it
#[derive(Clone)]
pub enum Derive<'a> {
    /// A [`Struct`]
    Struct(StructDeclaration<'a>),
}

impl<'a> Parse<'a> for Derive<'a> {
    fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
        let visibility = parser.parse()?;

        if parser.peek::<Struct>() {
            StructDeclaration::parse(visibility, parser).map(|r#struct| Derive::Struct(r#struct))
        } else {
            Err(parser.error("expected an expression"))
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
