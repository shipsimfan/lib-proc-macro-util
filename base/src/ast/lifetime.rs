use crate::{tokens::Identifier, Generator, Parse, Parser, Result, ToTokens, Token};

/// A lifetime indicator
#[derive(Debug, Clone)]
pub struct Lifetime {
    /// The ' indicating this is a lifetime
    pub apostrophe: Token!['_],

    /// The name of the lifetime
    pub name: Identifier,
}

impl<'a> Parse<'a> for Lifetime {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let apostrophe = parser.parse()?;
        let name = parser.parse()?;

        Ok(Lifetime { apostrophe, name })
    }
}

impl ToTokens for Lifetime {
    fn to_tokens(&self, generator: &mut Generator) {
        self.apostrophe.to_tokens(generator);
        self.name.to_tokens(generator);
    }
}
