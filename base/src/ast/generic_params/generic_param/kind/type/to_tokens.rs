use crate::{ast::TypeParam, Generator, ToTokens};

impl<'a> ToTokens for TypeParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.identifier.to_tokens(generator);
        self.bounds.to_tokens(generator);
        self.default.to_tokens(generator);
    }
}
