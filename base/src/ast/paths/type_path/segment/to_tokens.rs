use crate::{ast::TypePathSegment, Generator, ToTokens};

impl<'a> ToTokens for TypePathSegment<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.ident.to_tokens(generator);
        self.generics.to_tokens(generator);
    }
}
