use crate::{ast::LifetimeParam, Generator, ToTokens};

impl<'a> ToTokens for LifetimeParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.lifetime.to_tokens(generator);
        self.bounds.to_tokens(generator);
    }
}
