use crate::{ast::types::TraitObjectTypeOneBound, Generator, ToTokens};

impl<'a> ToTokens for TraitObjectTypeOneBound<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#dyn.to_tokens(generator);
        self.bound.to_tokens(generator);
    }
}
