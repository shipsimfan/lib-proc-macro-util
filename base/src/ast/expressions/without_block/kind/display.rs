use crate::ast::ExpressionWithoutBlockKind;

impl<'a> std::fmt::Display for ExpressionWithoutBlockKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => literal.fmt(f),
        }
    }
}
