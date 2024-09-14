use crate::{
    tokens::{Apostrophe, Identifier},
    Generator, Parse, Parser, Result, Span, ToTokens, Token,
};

/// A lifetime indicator
#[derive(Debug, Clone)]
pub struct Lifetime {
    /// The ' indicating this is a lifetime
    pub apostrophe: Token!['_],

    /// The name of the lifetime
    pub name: Identifier,
}

impl Lifetime {
    /// Creates a new [`Lifetime`] with a `call_site` span
    pub fn new(name: Identifier) -> Self {
        Lifetime {
            apostrophe: Apostrophe::new_joint([Span::call_site()]),
            name,
        }
    }
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
