use crate::{ast::TypeParamBound, Generator, ToTokens};

impl<'a> ToTokens for TypeParamBound<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TypeParamBound::Lifetime(lifetime) => lifetime.to_tokens(generator),
            TypeParamBound::Trait(trait_bound) => trait_bound.to_tokens(generator),
        }
    }
}
