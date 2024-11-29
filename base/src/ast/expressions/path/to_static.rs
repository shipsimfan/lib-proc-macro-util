use crate::ast::expressions::PathExpression;

impl<'a> PathExpression<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PathExpression<'static> {
        match self {
            PathExpression::Normal(normal) => PathExpression::Normal(normal.into_static()),
            PathExpression::QualifiedPathInExpression(path_in_expression) => {
                PathExpression::QualifiedPathInExpression(path_in_expression.into_static())
            }
        }
    }
}
