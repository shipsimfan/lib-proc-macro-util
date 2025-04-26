use crate::ast::{
    expressions::{
        CallExpression, FieldExpression, LiteralExpression, MethodCallExpression,
        OperatorExpression, PathExpression,
    },
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

impl<'a> From<OperatorExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(operator: OperatorExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Operator(operator)
    }
}

impl<'a> From<CallExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(call: CallExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Call(call)
    }
}

impl<'a> From<FieldExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(field: FieldExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Field(field)
    }
}

impl<'a> From<MethodCallExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(method_call: MethodCallExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::MethodCall(method_call)
    }
}
