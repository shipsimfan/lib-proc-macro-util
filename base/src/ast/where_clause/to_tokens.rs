use crate::{ast::WhereClause, Generator, ToTokens};

impl<'a> ToTokens for WhereClause<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#where.to_tokens(generator);
        self.initial.to_tokens(generator);
        self.r#final.to_tokens(generator);
    }
}
