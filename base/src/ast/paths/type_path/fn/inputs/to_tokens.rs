use crate::{ast::TypePathFnInputs, Generator, ToTokens};

impl ToTokens for TypePathFnInputs {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.end.to_tokens(generator);
    }
}
