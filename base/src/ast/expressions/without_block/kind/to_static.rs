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
            ExpressionWithoutBlockKind::Field(field) => {
                ExpressionWithoutBlockKind::Field(field.into_static())
            }
            ExpressionWithoutBlockKind::Underscore(underscore) => {
                ExpressionWithoutBlockKind::Underscore(underscore)
            }
            ExpressionWithoutBlockKind::MethodCall(method_call) => {
                ExpressionWithoutBlockKind::MethodCall(method_call.into_static())
            }
            ExpressionWithoutBlockKind::Continue(r#continue) => {
                ExpressionWithoutBlockKind::Continue(r#continue.into_static())
            }
            ExpressionWithoutBlockKind::Break(r#break) => {
                ExpressionWithoutBlockKind::Break(r#break.into_static())
            }
            ExpressionWithoutBlockKind::Return(r#return) => {
                ExpressionWithoutBlockKind::Return(r#return.into_static())
            }
            ExpressionWithoutBlockKind::Grouped(grouped) => {
                ExpressionWithoutBlockKind::Grouped(grouped.into_static())
            }
            ExpressionWithoutBlockKind::Array(array) => {
                ExpressionWithoutBlockKind::Array(array.into_static())
            }
            ExpressionWithoutBlockKind::AsyncBlock(async_block) => {
                ExpressionWithoutBlockKind::AsyncBlock(async_block.into_static())
            }
            ExpressionWithoutBlockKind::Index(index) => {
                ExpressionWithoutBlockKind::Index(index.into_static())
            }
            ExpressionWithoutBlockKind::Await(r#await) => {
                ExpressionWithoutBlockKind::Await(r#await.into_static())
            }
            ExpressionWithoutBlockKind::TupleIndex(tuple_index) => {
                ExpressionWithoutBlockKind::TupleIndex(tuple_index.into_static())
            }
        }
    }
}
