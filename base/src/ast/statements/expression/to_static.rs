use crate::ast::statements::ExpressionStatement;

impl<'a> ExpressionStatement<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionStatement<'static> {
        match self {
            ExpressionStatement::WithoutBlock(expression, comma) => {
                ExpressionStatement::WithoutBlock(expression.into_static(), comma)
            }
            ExpressionStatement::WithBlock(expression, comma) => {
                ExpressionStatement::WithBlock(expression.into_static(), comma)
            }
        }
    }
}
