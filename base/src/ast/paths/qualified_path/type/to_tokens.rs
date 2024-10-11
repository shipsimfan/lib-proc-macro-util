use crate::{ast::QualifiedPathType, Generator, ToTokens};

impl<'a> ToTokens for QualifiedPathType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.start.to_tokens(generator);
        self.r#type.to_tokens(generator);
        self.r#as.to_tokens(generator);
        self.end.to_tokens(generator);
    }
}
