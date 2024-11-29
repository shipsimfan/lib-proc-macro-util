use crate::ast::ExpressionWithoutBlockKind;

impl<'a> ExpressionWithoutBlockKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ExpressionWithoutBlockKind<'static> {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => {
                ExpressionWithoutBlockKind::Literal(literal.into_static())
            }
            ExpressionWithoutBlockKind::Path(path) => {
                ExpressionWithoutBlockKind::Path(path.into_static())
            }
        }
    }
}
