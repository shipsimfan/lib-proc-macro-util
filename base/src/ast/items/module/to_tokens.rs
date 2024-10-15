use crate::{ast::items::Module, Generator, ToTokens};

impl<'a> ToTokens for Module<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#unsafe.to_tokens(generator);
        self.r#mod.to_tokens(generator);
        self.identifier.clone().to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
