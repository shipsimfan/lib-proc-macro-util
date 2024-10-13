use crate::{ast::types::MaybeNamedParam, Generator, ToTokens};

impl<'a> ToTokens for MaybeNamedParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.name.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
