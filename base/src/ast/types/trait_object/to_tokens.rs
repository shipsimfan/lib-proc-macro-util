use crate::{ast::types::TraitObjectType, Generator, ToTokens};

impl<'a> ToTokens for TraitObjectType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#dyn.to_tokens(generator);
        self.bounds.to_tokens(generator);
    }
}
