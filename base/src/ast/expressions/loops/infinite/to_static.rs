use crate::ast::expressions::InfiniteLoopExpression;

impl<'a> InfiniteLoopExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> InfiniteLoopExpression<'static> {
        InfiniteLoopExpression {
            r#loop: self.r#loop,
            block: self.block.into_static(),
        }
    }
}
