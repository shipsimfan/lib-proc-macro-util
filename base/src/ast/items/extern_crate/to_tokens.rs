use crate::{ast::items::ExternCrate, Generator, ToTokens};

impl<'a> ToTokens for ExternCrate<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#extern.to_tokens(generator);
        self.krate.to_tokens(generator);
        self.crate_ref.to_tokens(generator);
        self.as_clause.to_tokens(generator);
    }
}
