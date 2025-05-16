use crate::ast::expressions::ArithmeticOrLogicalExpression;

impl<'a> ArithmeticOrLogicalExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ArithmeticOrLogicalExpression<'static> {
        ArithmeticOrLogicalExpression {
            left: Box::new(self.left.into_static()),
            operator: self.operator,
            right: Box::new(self.right.into_static()),
        }
    }
}
