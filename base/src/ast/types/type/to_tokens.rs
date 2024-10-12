use crate::{ast::Type, Generator, ToTokens};

impl ToTokens for Type {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Type::Parenthesized(parenthesized) => parenthesized.to_tokens(generator),
        }
    }
}
