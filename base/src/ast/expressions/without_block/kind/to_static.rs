use crate::ast::ExpressionWithoutBlockKind;

impl<'a> ExpressionWithoutBlockKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithoutBlockKind<'static> {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => {
                ExpressionWithoutBlockKind::Literal(literal.into_static())
            }
            ExpressionWithoutBlockKind::Path(path) => {
                ExpressionWithoutBlockKind::Path(path.into_static())
            }
            ExpressionWithoutBlockKind::MacroInvocation(macro_invocation) => {
                ExpressionWithoutBlockKind::MacroInvocation(macro_invocation.into_static())
            }
            ExpressionWithoutBlockKind::Operator(operator) => {
                ExpressionWithoutBlockKind::Operator(operator.into_static())
            }
            ExpressionWithoutBlockKind::Call(call) => {
                ExpressionWithoutBlockKind::Call(call.into_static())
            }
        }
    }
}
