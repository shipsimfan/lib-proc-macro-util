use crate::{ast::types::RawPointerType, Generator, ToTokens};

impl<'a> ToTokens for RawPointerType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.asterick.to_tokens(generator);
        self.mutability.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
