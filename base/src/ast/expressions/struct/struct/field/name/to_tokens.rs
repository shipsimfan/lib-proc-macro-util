use crate::{ast::expressions::StructExprFieldName, Generator, ToTokens};

impl<'a> ToTokens for StructExprFieldName<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            StructExprFieldName::Identifier(identifier) => identifier.to_tokens(generator),
            StructExprFieldName::Tuple(literal) => literal.to_tokens(generator),
        }
    }
}
