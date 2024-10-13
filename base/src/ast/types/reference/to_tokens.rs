use crate::{ast::types::ReferenceType, Generator, ToTokens};

impl<'a> ToTokens for ReferenceType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.reference.to_tokens(generator);
        self.lifetime.to_tokens(generator);
        self.r#mut.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
