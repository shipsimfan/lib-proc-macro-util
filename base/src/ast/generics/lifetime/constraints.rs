use crate::{
    ast::{Lifetime, Punctuated},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::GenericLifetime;

/// Constraints on a [`GenericLifetime`]
#[derive(Debug, Clone)]
pub struct GenericLifetimeConstraints {
    /// The colon indicating the start of the constraints
    pub colon: Token![:],

    /// The constraints on the lifetime
    pub constraints: Punctuated<Lifetime, Token![+]>,
}

impl<'a> Parse<'a> for GenericLifetimeConstraints {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let colon = parser.parse()?;
        let constraints = Punctuated::parse(parser, true, false)?;

        Ok(GenericLifetimeConstraints { colon, constraints })
    }
}

impl ToTokens for GenericLifetimeConstraints {
    fn to_tokens(&self, generator: &mut Generator) {
        self.colon.to_tokens(generator);
        self.constraints.to_tokens(generator);
    }
}
