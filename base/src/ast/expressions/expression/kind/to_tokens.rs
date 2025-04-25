use crate::{ast::ExpressionKind, Generator, ToTokens};

impl<'a> ToTokens for ExpressionKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionKind::Literal(literal) => literal.to_tokens(generator),
            ExpressionKind::Block(block) => block.to_tokens(generator),
            ExpressionKind::Path(path) => path.to_tokens(generator),
            ExpressionKind::MacroInvocation(macro_invocation) => {
                macro_invocation.to_tokens(generator)
            }
            ExpressionKind::Operator(operator) => operator.to_tokens(generator),
        }
    }
}
