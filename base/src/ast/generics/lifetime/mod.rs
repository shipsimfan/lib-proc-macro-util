use crate::{ast::Lifetime, Generator, Parse, Parser, Result, ToTokens};

mod constraints;

pub use constraints::GenericLifetimeConstraints;

/// A generic lifetime
#[derive(Clone)]
pub struct GenericLifetime {
    /// The name of the lifetime
    pub lifetime: Lifetime,

    /// The constraints on the lifetime
    pub constraints: Option<GenericLifetimeConstraints>,
}

impl<'a> Parse<'a> for GenericLifetime {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let lifetime = parser.parse()?;
        let constraints = parser.parse()?;

        Ok(GenericLifetime {
            lifetime,
            constraints,
        })
    }
}


impl ToTokens for GenericLifetime {
    fn to_tokens(&self, generator: &mut Generator) {
        self.lifetime.to_tokens(generator);
        self.constraints.to_tokens(generator);
    }
}
