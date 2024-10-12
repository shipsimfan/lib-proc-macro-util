use crate::{ast::types::ImplTraitTypeOneBound, Generator, ToTokens};

impl<'a> ToTokens for ImplTraitTypeOneBound<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#impl.to_tokens(generator);
        self.bound.to_tokens(generator);
    }
}
