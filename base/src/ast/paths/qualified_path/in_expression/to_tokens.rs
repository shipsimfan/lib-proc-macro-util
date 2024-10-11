use crate::{ast::QualifiedPathInExpression, Generator, ToTokens};

impl<'a> ToTokens for QualifiedPathInExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#type.to_tokens(generator);
        self.segments.to_tokens(generator);
    }
}
