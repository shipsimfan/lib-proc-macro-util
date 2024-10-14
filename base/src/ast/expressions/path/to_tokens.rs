use crate::{ast::expressions::PathExpression, Generator, ToTokens};

impl<'a> ToTokens for PathExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            PathExpression::Normal(path) => path.to_tokens(generator),
            PathExpression::QualifiedPathInExpression(qualified_path) => {
                qualified_path.to_tokens(generator)
            }
        }
    }
}
