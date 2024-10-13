use crate::{ast::types::FunctionTypeQualifiers, Generator, ToTokens};

impl<'a> ToTokens for FunctionTypeQualifiers<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#unsafe.to_tokens(generator);
        self.r#extern.to_tokens(generator);
    }
}
