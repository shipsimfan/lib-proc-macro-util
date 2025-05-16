use crate::ast::expressions::AsyncBlockExpression;

impl<'a> AsyncBlockExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> AsyncBlockExpression<'static> {
        AsyncBlockExpression {
            r#async: self.r#async,
            r#move: self.r#move,
            block: self.block.into_static(),
        }
    }
}
