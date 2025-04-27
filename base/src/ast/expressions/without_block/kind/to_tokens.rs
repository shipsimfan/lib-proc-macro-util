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
        }
    }
}
