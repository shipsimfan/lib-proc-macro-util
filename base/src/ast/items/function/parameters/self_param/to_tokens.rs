use crate::{ast::items::SelfParam, Generator, ToTokens};

impl<'a> ToTokens for SelfParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.reference.to_tokens(generator);
        self.r#mut.to_tokens(generator);
        self._self.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
