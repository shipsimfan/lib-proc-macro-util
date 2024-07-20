use crate::{tokens::Identifier, Generator, Parse, Parser, Result, ToTokens};

mod constraint;
mod constraints;

pub use constraint::GenericTypeConstraint;
pub use constraints::GenericTypeConstraints;

/// A generic type
#[derive(Clone)]
pub struct GenericType {
    /// The placeholder name for the type
    pub name: Identifier,

    /// The constraints on the type
    pub constraints: Option<GenericTypeConstraints>,
}

impl<'a> Parse<'a> for GenericType {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser.parse()?;
        let constraints = parser.parse()?;

        Ok(GenericType { name, constraints })
    }
}

impl ToTokens for GenericType {
    fn to_tokens(&self, generator: &mut Generator) {
        self.name.to_tokens(generator);
        self.constraints.to_tokens(generator);
    }
}
