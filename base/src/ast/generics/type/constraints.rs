use crate::{
    ast::{GenericTypeConstraint, Punctuated},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

/// A list of constraints on a generic type
#[derive(Clone)]
pub struct GenericTypeConstraints {
    /// The colon starting the constraints
    pub colon: Token![:],

    /// The constraints on the type
    pub constraints: Punctuated<GenericTypeConstraint, Token![+]>,
}

impl<'a> Parse<'a> for GenericTypeConstraints {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let colon = parser.parse()?;
        let constraints = Punctuated::parse(parser, true)?;

        Ok(GenericTypeConstraints { colon, constraints })
    }
}

impl ToTokens for GenericTypeConstraints {
    fn to_tokens(&self, generator: &mut Generator) {
        self.colon.to_tokens(generator);
        self.constraints.to_tokens(generator);
    }
}
