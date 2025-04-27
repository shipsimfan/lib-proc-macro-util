use crate::ast::expressions::ConstBlockExpression;

impl<'a> ConstBlockExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ConstBlockExpression<'static> {
        ConstBlockExpression {
            r#const: self.r#const,
            block: self.block.into_static(),
        }
    }
}
