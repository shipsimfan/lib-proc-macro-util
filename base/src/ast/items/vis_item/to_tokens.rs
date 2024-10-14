use crate::{ast::VisItem, Generator, ToTokens};

impl<'a> ToTokens for VisItem<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.visibility.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
