use crate::{ast::expressions::LoopExpressionKind, Generator, ToTokens};

impl<'a> ToTokens for LoopExpressionKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            LoopExpressionKind::Iterator(iterator) => iterator.to_tokens(generator),
        }
    }
}
