use crate::{
    ast::{GenericArgument, GenericLifetime, GenericType},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A single generic
#[derive(Debug, Clone)]
pub enum Generic {
    /// A generic lifetime
    Lifetime(GenericLifetime),

    /// A generic type
    Type(GenericType),
}

impl Generic {
    /// Convert this [`Generic`] to its corrisponding [`GenericArgument`]
    pub fn to_arg(&self) -> GenericArgument {
        match self {
            Generic::Lifetime(lifetime) => GenericArgument::Lifetime(lifetime.lifetime.clone()),
            Generic::Type(r#type) => GenericArgument::Type(r#type.name.clone().into()),
        }
    }
}

impl<'a> Parse<'a> for Generic {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token!['_]>() {
            parser.parse().map(|lifetime| Generic::Lifetime(lifetime))
        } else {
            parser.parse().map(|r#type| Generic::Type(r#type))
        }
    }
}

impl ToTokens for Generic {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            Generic::Lifetime(lifetime) => lifetime.to_tokens(generator),
            Generic::Type(r#type) => r#type.to_tokens(generator),
        }
    }
}
