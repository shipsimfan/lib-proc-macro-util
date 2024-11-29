use crate::ast::ExpressionWithoutBlockKind;

impl<'a> ExpressionWithoutBlockKind<'a> {
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
