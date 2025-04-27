use crate::{ast::patterns::StructPattern, Generator, ToTokens};

impl<'a> ToTokens for StructPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
        self.elements.to_tokens(&mut generator.group_brace());
    }
}
