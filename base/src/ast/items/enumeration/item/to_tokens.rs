use crate::{ast::items::EnumItem, Generator, ToTokens};

impl<'a> ToTokens for EnumItem<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.visibility.to_tokens(generator);
        self.name.to_tokens(generator);
        self.kind.to_tokens(generator);
        self.discriminant.to_tokens(generator);
    }
}
