use crate::{ast::Statement, Generator, ToTokens};

impl<'a> ToTokens for Statement<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Statement::Item(item) => item.to_tokens(generator),
        }
    }
}
