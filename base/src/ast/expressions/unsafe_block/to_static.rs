use crate::ast::expressions::UnsafeBlockExpression;

impl<'a> UnsafeBlockExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> UnsafeBlockExpression<'static> {
        UnsafeBlockExpression {
            r#unsafe: self.r#unsafe,
            block: self.block.into_static(),
        }
    }
}
