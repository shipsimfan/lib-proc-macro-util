use crate::{ast::expressions::StructExprKind, Generator, ToTokens};

impl<'a> ToTokens for StructExprKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            StructExprKind::Struct(r#struct) => r#struct.to_tokens(&mut generator.group_brace()),
            StructExprKind::Tuple(tuple) => tuple.to_tokens(&mut generator.group_parenthesis()),
            StructExprKind::Unit => {}
        }
    }
}
