use crate::{ast::TypeNoBounds, Generator, ToTokens};

impl ToTokens for TypeNoBounds {
    fn to_tokens(self, generator: &mut Generator) {}
}
