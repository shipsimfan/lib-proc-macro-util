use crate::{ast::Expression, Generator, ToTokens};

impl ToTokens for Expression {
    fn to_tokens(self, generator: &mut Generator) {}
}
