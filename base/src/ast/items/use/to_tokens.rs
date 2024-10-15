use crate::{ast::items::UseDeclaration, Generator, ToTokens};

impl<'a> ToTokens for UseDeclaration<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#use.to_tokens(generator);
        self.tree.to_tokens(generator);
        self.semi.to_tokens(generator);
    }
}
