use crate::ast::{
    expressions::{
        BlockExpression, CallExpression, LiteralExpression, OperatorExpression, PathExpression,
    },
    ExpressionKind, MacroInvocation,
};

impl<'a> From<LiteralExpression<'a>> for ExpressionKind<'a> {
    fn from(literal: LiteralExpression<'a>) -> Self {
        ExpressionKind::Literal(literal)
    }
}

impl<'a> From<BlockExpression<'a>> for ExpressionKind<'a> {
    fn from(block: BlockExpression<'a>) -> Self {
        ExpressionKind::Block(block)
    }
}

impl<'a> From<PathExpression<'a>> for ExpressionKind<'a> {
    fn from(path: PathExpression<'a>) -> Self {
        ExpressionKind::Path(path)
    }
}

impl<'a> From<MacroInvocation<'a>> for ExpressionKind<'a> {
    fn from(macro_invocation: MacroInvocation<'a>) -> Self {
        ExpressionKind::MacroInvocation(macro_invocation)
    }
}

impl<'a> From<OperatorExpression<'a>> for ExpressionKind<'a> {
    fn from(operator: OperatorExpression<'a>) -> Self {
        ExpressionKind::Operator(operator)
    }
}

impl<'a> From<CallExpression<'a>> for ExpressionKind<'a> {
    fn from(call: CallExpression<'a>) -> Self {
        ExpressionKind::Call(call)
    }
}
