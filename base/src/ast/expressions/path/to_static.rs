use crate::ast::expressions::PathExpression;

impl<'a> PathExpression<'a> {
    pub fn into_static(self) -> PathExpression<'static> {
        match self {
            PathExpression::Normal(normal) => PathExpression::Normal(normal.into_static()),
            PathExpression::QualifiedPathInExpression(path_in_expression) => {
                PathExpression::QualifiedPathInExpression(path_in_expression.into_static())
            }
        }
    }
}
