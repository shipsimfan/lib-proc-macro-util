use crate::{ast::items::EnumItemDiscriminant, Generator, ToTokens};

impl<'a> ToTokens for EnumItemDiscriminant<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.equals.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
