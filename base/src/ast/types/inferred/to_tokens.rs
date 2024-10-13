use crate::{ast::types::InferredType, Generator, ToTokens};

impl ToTokens for InferredType {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
    }
}
