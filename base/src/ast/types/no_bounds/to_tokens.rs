use crate::{ast::TypeNoBounds, Generator, ToTokens};

impl<'a> ToTokens for TypeNoBounds<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TypeNoBounds::Parenthesized(parenthesized) => parenthesized.to_tokens(generator),
            TypeNoBounds::ImplTraitOneBound(impl_trait) => impl_trait.to_tokens(generator),
        }
    }
}
