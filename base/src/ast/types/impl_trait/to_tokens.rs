use crate::{ast::types::ImplTraitType, Generator, ToTokens};

impl<'a> ToTokens for ImplTraitType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#impl.to_tokens(generator);
        self.bounds.to_tokens(generator);
    }
}
