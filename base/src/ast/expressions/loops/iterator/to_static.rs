use crate::ast::expressions::IteratorLoopExpression;

impl<'a> IteratorLoopExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> IteratorLoopExpression<'static> {
        IteratorLoopExpression {
            r#for: self.r#for,
            pattern: self.pattern.into_static(),
            r#in: self.r#in,
            iterator: Box::new(self.iterator.into_static()),
            block: self.block.into_static(),
        }
    }
}
