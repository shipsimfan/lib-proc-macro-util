use crate::{ast::TypeNoBounds, Generator, ToTokens};

impl ToTokens for TypeNoBounds {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TypeNoBounds::Parenthesized(parenthesized) => parenthesized.to_tokens(generator),
        }
    }
}
