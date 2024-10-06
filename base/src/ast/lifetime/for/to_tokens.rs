use crate::{ast::ForLifetimes, Generator, ToTokens};

impl<'a> ToTokens for ForLifetimes<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#for.to_tokens(generator);
        self.generics.to_tokens(generator);
    }
}
