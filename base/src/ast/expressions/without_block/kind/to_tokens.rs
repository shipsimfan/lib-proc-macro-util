use crate::{ast::ExpressionWithoutBlockKind, Generator, ToTokens};

impl<'a> ToTokens for ExpressionWithoutBlockKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => literal.to_tokens(generator),
            ExpressionWithoutBlockKind::Path(path) => path.to_tokens(generator),
        }
    }
}
