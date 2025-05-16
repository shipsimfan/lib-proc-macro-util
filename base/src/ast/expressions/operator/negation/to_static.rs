use crate::ast::expressions::NegationExpression;

impl<'a> NegationExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> NegationExpression<'static> {
        NegationExpression {
            operator: self.operator,
            expression: Box::new(self.expression.into_static()),
        }
    }
}
