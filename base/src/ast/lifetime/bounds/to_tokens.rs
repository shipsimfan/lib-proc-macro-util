use crate::{ast::LifetimeBounds, Generator, ToTokens};

impl<'a> ToTokens for LifetimeBounds<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.leading.to_tokens(generator);
        self.ending.to_tokens(generator);
    }
}
