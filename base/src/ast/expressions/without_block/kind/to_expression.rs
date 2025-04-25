use crate::ast::{ExpressionKind, ExpressionWithoutBlockKind};

impl<'a> ExpressionWithoutBlockKind<'a> {
    /// Convert this into a full [`ExpressionKind`]
    pub fn into_expression(self) -> ExpressionKind<'a> {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => ExpressionKind::Literal(literal),
            ExpressionWithoutBlockKind::Path(path) => ExpressionKind::Path(path),
            ExpressionWithoutBlockKind::MacroInvocation(macro_invocation) => {
                ExpressionKind::MacroInvocation(macro_invocation)
            }
            ExpressionWithoutBlockKind::Operator(operator) => ExpressionKind::Operator(operator),
            ExpressionWithoutBlockKind::Call(call) => ExpressionKind::Call(call),
        }
    }
}
