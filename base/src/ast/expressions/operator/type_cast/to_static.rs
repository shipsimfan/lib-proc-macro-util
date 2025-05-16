use crate::ast::expressions::TypeCastExpression;

impl<'a> TypeCastExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypeCastExpression<'static> {
        TypeCastExpression {
            expression: Box::new(self.expression.into_static()),
            r#as: self.r#as,
            r#type: Box::new(self.r#type.into_static()),
        }
    }
}
