use crate::{ast::GenericArgsBounds, Generator, ToTokens};

impl<'a> ToTokens for GenericArgsBounds<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.identifier.to_tokens(generator);
        self.args.to_tokens(generator);
        self.colon.to_tokens(generator);
        self.bounds.to_tokens(generator);
    }
}
