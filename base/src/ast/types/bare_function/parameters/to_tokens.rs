use crate::{ast::types::MaybeNamedFunctionParameters, Generator, ToTokens};

impl<'a> ToTokens for MaybeNamedFunctionParameters<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.variadic.to_tokens(generator);
        self.ending.to_tokens(generator);
    }
}
