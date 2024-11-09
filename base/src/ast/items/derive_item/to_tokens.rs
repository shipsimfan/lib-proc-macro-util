use crate::{ast::items::DeriveItem, Generator, ToTokens};

impl<'a> ToTokens for DeriveItem<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.visibility.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
