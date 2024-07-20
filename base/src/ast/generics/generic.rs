use crate::{
    ast::{GenericLifetime, GenericType},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A single generic
#[derive(Clone)]
pub enum Generic {
    /// A generic lifetime
    Lifetime(GenericLifetime),

    /// A generic type
    Type(GenericType),
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
