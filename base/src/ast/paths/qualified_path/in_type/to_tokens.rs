use crate::{ast::QualifiedPathInType, Generator, ToTokens};

impl<'a> ToTokens for QualifiedPathInType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#type.to_tokens(generator);
        self.segments.to_tokens(generator);
    }
}
