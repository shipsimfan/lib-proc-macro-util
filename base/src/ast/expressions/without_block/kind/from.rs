use crate::ast::{
    expressions::{LiteralExpression, PathExpression},
    ExpressionWithoutBlockKind, MacroInvocation,
};

impl<'a> From<LiteralExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(literal: LiteralExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Literal(literal)
    }
}

impl<'a> From<PathExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(path: PathExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Path(path)
    }
}

impl<'a> From<MacroInvocation<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(macro_invocation: MacroInvocation<'a>) -> Self {
        ExpressionWithoutBlockKind::MacroInvocation(macro_invocation)
    }
}
