use crate::{
    ast::{Lifetime, Type},
    Generator, Parse, Parser, Result, ToTokens, Token,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::GenericType;

/// A constraint on a [`GenericType`]
#[derive(Debug, Clone)]
pub enum GenericTypeConstraint {
    /// The constraint is a lifetime
    Lifetime(Lifetime),

    /// The constraint is a trait
    Trait(Type),
}

impl<'a> Parse<'a> for GenericTypeConstraint {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token!['_]>() {
            parser
                .parse()
                .map(|lifetime| GenericTypeConstraint::Lifetime(lifetime))
        } else {
            parser
                .parse()
                .map(|r#trait| GenericTypeConstraint::Trait(r#trait))
        }
    }
}

impl ToTokens for GenericTypeConstraint {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            GenericTypeConstraint::Lifetime(lifetime) => lifetime.to_tokens(generator),
            GenericTypeConstraint::Trait(r#trait) => r#trait.to_tokens(generator),
        }
    }
}
