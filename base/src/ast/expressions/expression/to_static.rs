use crate::ast::Expression;

impl<'a> Expression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Expression<'static> {
        match self {
            Expression::WithBlock(expression) => Expression::WithBlock(expression.into_static()),
            Expression::WithoutBlock(expression) => {
                Expression::WithoutBlock(expression.into_static())
            }
        }
    }
}
