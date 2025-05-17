use crate::{ast::ExpressionWithoutBlockKind, Generator, ToTokens};

impl<'a> ToTokens for ExpressionWithoutBlockKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionWithoutBlockKind::Literal(literal) => literal.to_tokens(generator),
            ExpressionWithoutBlockKind::Path(path) => path.to_tokens(generator),
            ExpressionWithoutBlockKind::MacroInvocation(macro_invocation) => {
                macro_invocation.to_tokens(generator)
            }
            ExpressionWithoutBlockKind::Operator(operator) => operator.to_tokens(generator),
            ExpressionWithoutBlockKind::Call(call) => call.to_tokens(generator),
            ExpressionWithoutBlockKind::Field(field) => field.to_tokens(generator),
            ExpressionWithoutBlockKind::Underscore(underscore) => underscore.to_tokens(generator),
            ExpressionWithoutBlockKind::MethodCall(method_call) => method_call.to_tokens(generator),
            ExpressionWithoutBlockKind::Continue(r#continue) => r#continue.to_tokens(generator),
            ExpressionWithoutBlockKind::Break(r#break) => r#break.to_tokens(generator),
            ExpressionWithoutBlockKind::Return(r#return) => r#return.to_tokens(generator),
            ExpressionWithoutBlockKind::Grouped(grouped) => grouped.to_tokens(generator),
            ExpressionWithoutBlockKind::Array(array) => array.to_tokens(generator),
            ExpressionWithoutBlockKind::AsyncBlock(async_block) => async_block.to_tokens(generator),
            ExpressionWithoutBlockKind::Index(index) => index.to_tokens(generator),
            ExpressionWithoutBlockKind::Await(r#await) => r#await.to_tokens(generator),
            ExpressionWithoutBlockKind::TupleIndex(tuple_index) => tuple_index.to_tokens(generator),
            ExpressionWithoutBlockKind::Closure(closure) => closure.to_tokens(generator),
            ExpressionWithoutBlockKind::Range(range) => range.to_tokens(generator),
            ExpressionWithoutBlockKind::Tuple(tuple) => tuple.to_tokens(generator),
        }
    }
}
