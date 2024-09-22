use crate::{ast::Statement, Generator, ToTokens};

impl ToTokens for Statement {
    fn to_tokens(self, _: &mut Generator) {}
}
