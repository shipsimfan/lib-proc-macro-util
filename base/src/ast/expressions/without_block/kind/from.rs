use crate::ast::{
    expressions::{
        ArrayExpression, AsyncBlockExpression, AwaitExpression, BreakExpression, CallExpression,
        ContinueExpression, FieldExpression, GroupedExpression, IndexExpression, LiteralExpression,
        MethodCallExpression, OperatorExpression, PathExpression, ReturnExpression,
        TupleIndexExpression, UnderscoreExpression,
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

impl<'a> From<UnderscoreExpression> for ExpressionWithoutBlockKind<'a> {
    fn from(underscore: UnderscoreExpression) -> Self {
        ExpressionWithoutBlockKind::Underscore(underscore)
    }
}

impl<'a> From<MethodCallExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(method_call: MethodCallExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::MethodCall(method_call)
    }
}

impl<'a> From<ContinueExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(r#continue: ContinueExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Continue(r#continue)
    }
}

impl<'a> From<BreakExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(r#break: BreakExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Break(r#break)
    }
}

impl<'a> From<ReturnExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(r#return: ReturnExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Return(r#return)
    }
}

impl<'a> From<GroupedExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(grouped: GroupedExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Grouped(grouped)
    }
}

impl<'a> From<ArrayExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(array: ArrayExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Array(array)
    }
}

impl<'a> From<AsyncBlockExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(async_block: AsyncBlockExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::AsyncBlock(async_block)
    }
}

impl<'a> From<IndexExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(index: IndexExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Index(index)
    }
}

impl<'a> From<AwaitExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(r#await: AwaitExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::Await(r#await)
    }
}

impl<'a> From<TupleIndexExpression<'a>> for ExpressionWithoutBlockKind<'a> {
    fn from(tuple_index: TupleIndexExpression<'a>) -> Self {
        ExpressionWithoutBlockKind::TupleIndex(tuple_index)
    }
}
