use crate::{ast::ExpressionKind, Generator, ToTokens};

impl<'a> ToTokens for ExpressionKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionKind::Literal(literal) => literal.to_tokens(generator),
        }
    }
}
