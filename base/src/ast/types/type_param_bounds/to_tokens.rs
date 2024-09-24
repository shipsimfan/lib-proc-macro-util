use crate::{ast::TypeParamBounds, Generator, ToTokens};

impl<'a> ToTokens for TypeParamBounds<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.end.to_tokens(generator);
    }
}
