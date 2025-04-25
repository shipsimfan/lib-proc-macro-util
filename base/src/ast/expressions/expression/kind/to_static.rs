use crate::ast::ExpressionKind;

impl<'a> ExpressionKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionKind<'static> {
        match self {
            ExpressionKind::Literal(literal) => ExpressionKind::Literal(literal.into_static()),
            ExpressionKind::Block(block) => ExpressionKind::Block(block.into_static()),
            ExpressionKind::Path(path) => ExpressionKind::Path(path.into_static()),
            ExpressionKind::MacroInvocation(macro_invocation) => {
                ExpressionKind::MacroInvocation(macro_invocation.into_static())
            }
            ExpressionKind::Operator(operator) => ExpressionKind::Operator(operator.into_static()),
            ExpressionKind::Call(call) => ExpressionKind::Call(call.into_static()),
            ExpressionKind::Unsafe(r#unsafe) => ExpressionKind::Unsafe(r#unsafe.into_static()),
        }
    }
}
